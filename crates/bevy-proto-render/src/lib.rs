pub mod render_resource;
pub mod renderer;
pub mod settings;
//todo delete allow
#[allow(dead_code)]
pub mod sync_world;

use bevy_app::{App, AppLabel, Plugin, SubApp};
use bevy_asset::AssetServer;
use bevy_ecs::{
    prelude::Resource,
    query::With,
    schedule::{
        IntoScheduleConfigs, Schedule, ScheduleBuildSettings, ScheduleLabel, Schedules, SystemSet,
    },
    world::{Mut, World},
};
use bevy_window::{PrimaryWindow, RawHandleWrapperHolder};
use settings::{RenderCreation, RenderResources};
use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
};
use sync_world::{SyncWorldPlugin, despawn_temporary_render_entities, entity_sync_system};

#[derive(Resource)]
struct FutureRenderResources(Arc<Mutex<Option<RenderResources>>>);

#[derive(Default)]
pub struct ProtoRenderPlugin {
    pub render_creation: RenderCreation,
}

/// A label for the rendering sub-app.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, AppLabel)]
pub struct RenderApp;

impl Plugin for ProtoRenderPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        match &self.render_creation {
            RenderCreation::Manual(resources) => {
                let future_render_resources_wrapper = Arc::new(Mutex::new(Some(resources.clone())));
                app.insert_resource(FutureRenderResources(
                    future_render_resources_wrapper.clone(),
                ));
                // SAFETY: Plugins should be set up on the main thread.
                unsafe { initialize_render_app(app) };
            }
            RenderCreation::Automatic(render_creation) => {
                if let Some(backends) = render_creation.backends {
                    let future_render_resources_wrapper = Arc::new(Mutex::new(None));
                    app.insert_resource(FutureRenderResources(
                        future_render_resources_wrapper.clone(),
                    ));

                    let primary_window = app
                        .world_mut()
                        .query_filtered::<&RawHandleWrapperHolder, With<PrimaryWindow>>()
                        .single(app.world())
                        .ok()
                        .cloned();
                    let settings = render_creation.clone();
                    let async_renderer = async move {
                        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
                            backends,
                            flags: settings.instance_flags,
                            backend_options: wgpu::BackendOptions {
                                gl: wgpu::GlBackendOptions {
                                    gles_minor_version: settings.gles3_minor_version,
                                },
                                dx12: wgpu::Dx12BackendOptions {
                                    shader_compiler: settings.dx12_shader_compiler.clone(),
                                },
                            },
                        });

                        let surface = primary_window.and_then(|wrapper| {
                            let maybe_handle = wrapper.0.lock().expect(
                                "Couldn't get the window handle in time for renderer initialization",
                            );
                            if let Some(wrapper) = maybe_handle.as_ref() {
                                // SAFETY: Plugins should be set up on the main thread.
                                let handle = unsafe { wrapper.get_handle() };
                                Some(
                                    instance
                                        .create_surface(handle)
                                        .expect("Failed to create wgpu surface"),
                                )
                            } else {
                                None
                            }
                        });

                        let request_adapter_options = wgpu::RequestAdapterOptions {
                            power_preference: settings.power_preference,
                            compatible_surface: surface.as_ref(),
                            ..Default::default()
                        };

                        let (device,) = renderer::initialize_renderer(
                            &instance,
                            &settings,
                            &request_adapter_options,
                        )
                        .await;
                        let mut future_render_resources_inner =
                            future_render_resources_wrapper.lock().unwrap();
                        *future_render_resources_inner = Some(RenderResources(
                            device,
                            // queue,
                            // adapter_info,
                            // render_adapter,
                            // RenderInstance(Arc::new(WgpuWrapper::new(instance))),
                        ));
                    };
                    // In wasm, spawn a task and detach it for execution
                    #[cfg(target_arch = "wasm32")]
                    bevy_tasks::IoTaskPool::get()
                        .spawn_local(async_renderer)
                        .detach();
                    // Otherwise, just block for it to complete
                    #[cfg(not(target_arch = "wasm32"))]
                    futures_lite::future::block_on(async_renderer);

                    // SAFETY: Plugins should be set up on the main thread.
                    unsafe { initialize_render_app(app) };
                }
            }
        };

        app.add_plugins(SyncWorldPlugin);
    }

    fn ready(&self, app: &App) -> bool {
        app.world()
            .get_resource::<FutureRenderResources>()
            .and_then(|frr| frr.0.try_lock().map(|locked| locked.is_some()).ok())
            .unwrap_or(true)
    }

    fn finish(&self, app: &mut App) {
        if let Some(future_render_resources) =
            app.world_mut().remove_resource::<FutureRenderResources>()
        {
            let RenderResources(device) = future_render_resources.0.lock().unwrap().take().unwrap();

            app.insert_resource(device.clone());

            let render_app = app.sub_app_mut(RenderApp);
            render_app.insert_resource(device);
        }
    }
}

/// The systems sets of the default [`App`] rendering schedule.
///
/// These can be useful for ordering, but you almost never want to add your systems to these sets.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum RenderSet {
    /// This is used for applying the commands from the [`ExtractSchedule`]
    ExtractCommands,
    /// Prepare assets that have been created/modified/removed this frame.
    PrepareAssets,
    /// Prepares extracted meshes.
    PrepareMeshes,
    /// Create any additional views such as those used for shadow mapping.
    ManageViews,
    /// Queue drawable entities as phase items in render phases ready for
    /// sorting (if necessary)
    Queue,
    /// A sub-set within [`Queue`](RenderSet::Queue) where mesh entity queue systems are executed. Ensures `prepare_assets::<RenderMesh>` is completed.
    QueueMeshes,
    /// A sub-set within [`Queue`](RenderSet::Queue) where meshes that have
    /// become invisible or changed phases are removed from the bins.
    QueueSweep,
    // TODO: This could probably be moved in favor of a system ordering
    // abstraction in `Render` or `Queue`
    /// Sort the [`SortedRenderPhase`](render_phase::SortedRenderPhase)s and
    /// [`BinKey`](render_phase::BinnedPhaseItem::BinKey)s here.
    PhaseSort,
    /// Prepare render resources from extracted data for the GPU based on their sorted order.
    /// Create [`BindGroups`](render_resource::BindGroup) that depend on those data.
    Prepare,
    /// A sub-set within [`Prepare`](RenderSet::Prepare) for initializing buffers, textures and uniforms for use in bind groups.
    PrepareResources,
    /// Collect phase buffers after
    /// [`PrepareResources`](RenderSet::PrepareResources) has run.
    PrepareResourcesCollectPhaseBuffers,
    /// Flush buffers after [`PrepareResources`](RenderSet::PrepareResources), but before [`PrepareBindGroups`](RenderSet::PrepareBindGroups).
    PrepareResourcesFlush,
    /// A sub-set within [`Prepare`](RenderSet::Prepare) for constructing bind groups, or other data that relies on render resources prepared in [`PrepareResources`](RenderSet::PrepareResources).
    PrepareBindGroups,
    ///Setup FrameGraph
    FrameGraphSetup,
    /// Actual rendering happens here.
    /// In most cases, only the render backend should insert resources here.
    Render,
    /// Cleanup render resources here.
    Cleanup,
    /// Final cleanup occurs: all entities will be despawned.
    ///
    /// Runs after [`Cleanup`](RenderSet::Cleanup).
    PostCleanup,
}

/// The main render schedule.
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub struct Render;

impl Render {
    /// Sets up the base structure of the rendering [`Schedule`].
    ///
    /// The sets defined in this enum are configured to run in order.
    pub fn base_schedule() -> Schedule {
        use RenderSet::*;

        let mut schedule = Schedule::new(Self);

        schedule.configure_sets(
            (
                ExtractCommands,
                PrepareMeshes,
                ManageViews,
                Queue,
                PhaseSort,
                Prepare,
                FrameGraphSetup,
                Render,
                Cleanup,
                PostCleanup,
            )
                .chain(),
        );

        schedule.configure_sets((ExtractCommands, PrepareAssets, PrepareMeshes, Prepare).chain());
        schedule.configure_sets((QueueMeshes, QueueSweep).chain().in_set(Queue));
        schedule.configure_sets(
            (
                PrepareResources,
                PrepareResourcesCollectPhaseBuffers,
                PrepareResourcesFlush,
                PrepareBindGroups,
            )
                .chain()
                .in_set(Prepare),
        );

        schedule
    }
}

/// # Safety
/// This function must be called from the main thread.
unsafe fn initialize_render_app(app: &mut App) {
    app.init_resource::<ScratchMainWorld>();

    let mut render_app = SubApp::new();
    render_app.update_schedule = Some(Render.intern());

    let mut extract_schedule = Schedule::new(ExtractSchedule);
    // We skip applying any commands during the ExtractSchedule
    // so commands can be applied on the render thread.
    extract_schedule.set_build_settings(ScheduleBuildSettings {
        auto_insert_apply_deferred: false,
        ..Default::default()
    });
    extract_schedule.set_apply_final_deferred(false);

    render_app
        .add_schedule(extract_schedule)
        .add_schedule(Render::base_schedule())
        .insert_resource(app.world().resource::<AssetServer>().clone())
        .add_systems(
            Render,
            (
                // This set applies the commands from the extract schedule while the render schedule
                // is running in parallel with the main app.
                apply_extract_commands.in_set(RenderSet::ExtractCommands),
                despawn_temporary_render_entities.in_set(RenderSet::PostCleanup),
            ),
        );

    render_app.set_extract(|main_world, render_world| {
        {
            #[cfg(feature = "trace")]
            let _stage_span = tracing::info_span!("entity_sync").entered();
            entity_sync_system(main_world, render_world);
        }

        // run extract schedule
        extract(main_world, render_world);
    });

    let (sender, receiver) = bevy_time::create_time_channels();
    render_app.insert_resource(sender);
    app.insert_resource(receiver);
    app.insert_sub_app(RenderApp, render_app);
}

/// Applies the commands from the extract schedule. This happens during
/// the render schedule rather than during extraction to allow the commands to run in parallel with the
/// main app when pipelined rendering is enabled.
fn apply_extract_commands(render_world: &mut World) {
    render_world.resource_scope(|render_world, mut schedules: Mut<Schedules>| {
        schedules
            .get_mut(ExtractSchedule)
            .unwrap()
            .apply_deferred(render_world);
    });
}

#[derive(Resource, Default)]
struct ScratchMainWorld(World);

/// Schedule which extract data from the main world and inserts it into the render world.
///
/// This step should be kept as short as possible to increase the "pipelining potential" for
/// running the next frame while rendering the current frame.
///
/// This schedule is run on the main world, but its buffers are not applied
/// until it is returned to the render world.
#[derive(ScheduleLabel, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub struct ExtractSchedule;

/// Executes the [`ExtractSchedule`] step of the renderer.
/// This updates the render world with the extracted ECS data of the current frame.
fn extract(main_world: &mut World, render_world: &mut World) {
    // temporarily add the app world to the render world as a resource
    let scratch_world = main_world.remove_resource::<ScratchMainWorld>().unwrap();
    let inserted_world = core::mem::replace(main_world, scratch_world.0);
    render_world.insert_resource(MainWorld(inserted_world));
    render_world.run_schedule(ExtractSchedule);

    // move the app world back, as if nothing happened.
    let inserted_world = render_world.remove_resource::<MainWorld>().unwrap();
    let scratch_world = core::mem::replace(main_world, inserted_world.0);
    main_world.insert_resource(ScratchMainWorld(scratch_world));
}

/// The simulation [`World`] of the application, stored as a resource.
///
/// This resource is only available during [`ExtractSchedule`] and not
/// during command application of that schedule.
/// See [`Extract`] for more details.
#[derive(Resource, Default)]
pub struct MainWorld(World);

impl Deref for MainWorld {
    type Target = World;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MainWorld {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

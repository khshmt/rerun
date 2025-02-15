use ahash::HashMap;
use parking_lot::RwLock;

use re_entity_db::{entity_db::EntityDb, EntityTree, TimeHistogramPerTimeline};

use crate::{
    query_context::DataQueryResult, AppOptions, ApplicableEntities, ApplicationSelectionState,
    Caches, CommandSender, ComponentUiRegistry, DataQueryId, IndicatorMatchingEntities,
    PerVisualizer, Selection, SpaceViewClassRegistry, StoreContext, TimeControl,
};

/// Common things needed by many parts of the viewer.
pub struct ViewerContext<'a> {
    /// Global options for the whole viewer.
    pub app_options: &'a AppOptions,

    /// Things that need caching and are shared across the whole viewer.
    ///
    /// Use this only for things that you expected be shared across different panels and/or space views.
    pub cache: &'a Caches,

    /// How to display components.
    pub component_ui_registry: &'a ComponentUiRegistry,

    /// Registry of all known classes of space views.
    pub space_view_class_registry: &'a SpaceViewClassRegistry,

    /// The current recording.
    /// TODO(jleibs): This can go away
    pub entity_db: &'a EntityDb,

    /// The current view of the store
    pub store_context: &'a StoreContext<'a>,

    /// Mapping from class and system to entities for the store
    ///
    /// TODO(andreas): This should have a generation id, allowing to update heuristics(?)/visualizable_entities etc.
    pub applicable_entities_per_visualizer: &'a PerVisualizer<ApplicableEntities>,

    /// For each visualizer, the set of entities that have at least one matching indicator component.
    pub indicator_matching_entities_per_visualizer: &'a PerVisualizer<IndicatorMatchingEntities>,

    /// All the query results for this frame
    pub query_results: &'a HashMap<DataQueryId, DataQueryResult>,

    /// UI config for the current recording (found in [`EntityDb`]).
    pub rec_cfg: &'a RecordingConfig,

    /// The look and feel of the UI.
    pub re_ui: &'a re_ui::ReUi,

    /// The global `re_renderer` context, holds on to all GPU resources.
    pub render_ctx: &'a re_renderer::RenderContext,

    /// Interface for sending commands back to the app
    pub command_sender: &'a CommandSender,
}

impl<'a> ViewerContext<'a> {
    /// Returns the current selection.
    pub fn selection(&self) -> &Selection {
        self.rec_cfg.selection_state.current()
    }

    /// Returns the currently hovered objects.
    pub fn hovered(&self) -> &Selection {
        self.rec_cfg.selection_state.hovered()
    }

    pub fn selection_state(&self) -> &ApplicationSelectionState {
        &self.rec_cfg.selection_state
    }

    /// The current time query, based on the current time control.
    pub fn current_query(&self) -> re_data_store::LatestAtQuery {
        self.rec_cfg.time_ctrl.read().current_query()
    }

    /// Returns whether the given tree has any data logged in the current timeline,
    /// or has any timeless messages.
    pub fn tree_has_data_in_current_timeline(&self, tree: &EntityTree) -> bool {
        let top_time_histogram = &tree.subtree.time_histogram;
        top_time_histogram.has_timeline(self.rec_cfg.time_ctrl.read().timeline())
            || top_time_histogram.num_timeless_messages() > 0
    }

    /// Returns whether the given component has any data logged in the current timeline.
    pub fn component_has_data_in_current_timeline(
        &self,
        component_stat: &TimeHistogramPerTimeline,
    ) -> bool {
        component_stat.has_timeline(self.rec_cfg.time_ctrl.read().timeline())
            || component_stat.num_timeless_messages() > 0
    }
}

// ----------------------------------------------------------------------------

/// UI config for the current recording (found in [`EntityDb`]).
#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct RecordingConfig {
    /// The current time of the time panel, how fast it is moving, etc.
    pub time_ctrl: RwLock<TimeControl>,

    /// Selection & hovering state.
    pub selection_state: ApplicationSelectionState,
}

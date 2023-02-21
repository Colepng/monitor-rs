use hyprland::data::Monitors;
use hyprland::event_listener::EventListenerMutable as EventListener;
use hyprland::shared::HyprData;
use hyprland::keyword::Keyword;
use hyprland::dispatch::*;

fn MoveWorkspace(workspace: i32, monitor: &str) {
    Dispatch::call(DispatchType::MoveWorkspaceToMonitor(
        WorkspaceIdentifier::Id(workspace),
        MonitorIdentifier::Name(monitor)
    )); 
}

fn main() -> hyprland::shared::HResult<()> {
    let monitors = Monitors::get();
    let mut monitor_1: bool = false;
    println!("{monitors:#?}");

    let mut event_listener = EventListener::new();

    event_listener.add_monitor_added_handler(|monitor, _|{
        if monitor == *"HDMI-A-1".to_string() {
            Keyword::set("monitor", "HDMI-A-1, 1680X1050, 1920x0, 1");
            Dispatch::call(DispatchType::FocusMonitor(MonitorIdentifier::Name("eDP-1")));
            // Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Id(10)));
            Keyword::set("monitor", "eDP-1, disable");
            MoveWorkspace(3, "HDMI-A-1");
            MoveWorkspace(4, "HDMI-A-1");
            MoveWorkspace(5, "HDMI-A-1");
        }
        else if monitor == *"DP-2".to_string() {
            Keyword::set("monitor", "DP-2, 1920x1080@144, 0x0, 1");
            Dispatch::call(DispatchType::FocusMonitor(MonitorIdentifier::Name("eDP-1")));
            // Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Id(10)));
            Keyword::set("monitor", "eDP-1, disable");
            MoveWorkspace(1, "DP-2");
            MoveWorkspace(2, "DP-2");
        }
        // else if monitor != *"eDP-1".to_string() {
        //     Keyword::set("monitor", "eDP-1, 2560x1600, 0x0, 1.5");
        //     Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Id(10)));
        // }
        println!("monitor added")
    });

    event_listener.add_monitor_removed_handler(|monitor, _| {
        if monitor != *"eDP-1".to_string() {
            Keyword::set("monitor", "eDP-1, 2560x1600, 0x0, 1.5");
            MoveWorkspace(1, "eDP-1");
            MoveWorkspace(2, "eDP-1");
            MoveWorkspace(3, "eDP-1");
            MoveWorkspace(4, "eDP-1");
            MoveWorkspace(5, "eDP-1");
            MoveWorkspace(6, "eDP-1");
            MoveWorkspace(7, "eDP-1");
            MoveWorkspace(8, "eDP-1");
            MoveWorkspace(9, "eDP-1");
            // Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Id(10)));
        }
    });
    //
    event_listener.add_workspace_moved_handler(|id, _| println!("workspace was moved: {id:#?}"));

    event_listener.start_listener()
}

//! A library for controlling i3-wm through its ipc interface.

pub mod reply;

pub enum Event {
    // none of these are complete
    Workspace,
    Output,
    Mode,
    Window,
    BarConfigUpdate,
    Binding
}

pub struct I3Connection;

impl I3Connection {
    /// Establishes an IPC connection to i3.
    pub fn connect() -> Result<I3Connection, String> {
        panic!("not implemented");
    }

    /// The payload of the message is a command for i3 (like the commands you can bind to keys
    /// in the configuration file) and will be executed directly after receiving it.
    pub fn command(&mut self, string: &str) -> Result<reply::Command, String> {
        panic!("not implemented");
    }

    /// Gets the current workspaces.
    pub fn get_workspaces(&mut self) -> Result<reply::Workspaces, String> {
        panic!("not implemented");
    }

    /// Subscribes your connection to certain events.
    pub fn subscribe(&mut self) -> Result<reply::Subscribe, String> {
        panic!("not implemented");
    }

    /// Gets the current outputs.
    pub fn get_outputs(&mut self) -> Result<reply::Outputs, String> {
        panic!("not implemented");
    }

    /// Gets the layout tree. i3 uses a tree as data structure which includes every container.
    pub fn get_tree(&mut self) -> Result<reply::Tree, String> {
        panic!("not implemented");
    }

    /// Gets a list of marks (identifiers for containers to easily jump to them later).
    pub fn get_marks(&mut self) -> Result<reply::Marks, String> {
        panic!("not implemented");
    }

    /// Gets an array with all configured bar IDs.
    pub fn get_bar_ids(&mut self) -> Result<reply::BarIds, String> {
        panic!("not implemented");
    }

    /// Gets the configuration of the workspace bar with the given ID.
    pub fn get_bar_config(&mut self, id: &str) -> Result<reply::BarConfig, String> {
        panic!("not implemented");
    }

    /// Gets the version of i3. The reply will include the major, minor, patch and human-readable
    /// version.
    pub fn get_version(&mut self) -> Result<reply::Version, String> {
        panic!("not implemented");
    }
}


#[cfg(test)]
mod test {
    use I3Connection;

    // for the following tests send a request and get the reponse.
    // response types are specific so often getting them at all indicates success.
    // can't do much better without mocking an i3 installation.

    #[test]
    fn command_nothing() {
        let mut connection = I3Connection::connect().unwrap();
        let result = connection.command("").unwrap();
        assert_eq!(result.outcomes.len(), 0);
    }

    #[test]
    fn command_single_sucess() {
        let mut connection = I3Connection::connect().unwrap();
        let a = connection.command("move scratchpad").unwrap();
        let b = connection.command("scratchpad show").unwrap();
        assert_eq!(a.outcomes.len(), 1);
        assert_eq!(b.outcomes.len(), 1);
        assert!(a.outcomes[0].success);
        assert!(b.outcomes[0].success);
    }

    #[test]
    fn command_multiple_success() {
        let mut connection = I3Connection::connect().unwrap();
        let result = connection.command("move scratchpad; scratchpad show").unwrap();
        assert_eq!(result.outcomes.len(), 2);
        assert!(result.outcomes[0].success);
        assert!(result.outcomes[1].success);
    }

    #[test]
    fn command_fail() {
        let mut connection = I3Connection::connect().unwrap();
        let result = connection.command("ThisIsClearlyNotACommand").unwrap();
        assert_eq!(result.outcomes.len(), 1);
        assert!(!result.outcomes[0].success);
    }

    #[test]
    fn get_workspaces() {
        I3Connection::connect().unwrap().get_workspaces().unwrap();
    }

    #[test]
    fn subscribe() {
        I3Connection::connect().unwrap().subscribe().unwrap();
    }

    #[test]
    fn get_outputs() {
        I3Connection::connect().unwrap().get_outputs().unwrap();
    }

    #[test]
    fn get_tree() {
        I3Connection::connect().unwrap().get_tree().unwrap();
    }

    #[test]
    fn get_marks() {
        I3Connection::connect().unwrap().get_marks().unwrap();
    }

    #[test]
    fn get_bar_ids() {
        I3Connection::connect().unwrap().get_bar_ids().unwrap();
    }

    #[test]
    fn get_bar_ids_and_one_config() {
        let mut connection = I3Connection::connect().unwrap();
        let ids = connection.get_bar_ids().unwrap().ids;
        let oneconfig = connection.get_bar_config(&ids[0]).unwrap();
    }

    #[test]
    fn get_version() {
        I3Connection::connect().unwrap().get_version().unwrap();
    }
}

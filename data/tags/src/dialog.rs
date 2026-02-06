use super::*;
pub fn get_dialog() -> HashMap<&'static str, Vec<&'static str>> {
	let mut output: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
	output.insert("pause_screen_additions", vec![]);
	output.insert("quick_actions", vec![]);
	return output;
}
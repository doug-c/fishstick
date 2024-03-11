/* Marco Polo Game

If the name Marco is given, the response should be Polo. Otherwise, the response should be What's your name?

*/

pub fn marco_polo(name: &str) -> String {
    if name == "Marco" {
        "Polo".to_string()
    } else {
        "What's your name?".to_string()
    }
}

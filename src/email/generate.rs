use crate::stories::story::Story;
use handlebars::Handlebars;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs;

pub fn render_newsletter(stories: &mut Vec<Story>) -> Result<String, Box<dyn std::error::Error>> {
    // Shuffle the stories
    shuffle_stories(stories);

    // Create a new Handlebars instance
    let mut handlebars = Handlebars::new();

    // Load up html template
    let template_filepath = fs::read_to_string("src/email/template.html")?;
    handlebars.register_template_string("newsletter", template_filepath)?;

    // Convert Story vector to JSON.
    let data = serde_json::json!({ "stories": stories });

    // Render template with data
    let rendered = handlebars.render("newsletter", &data)?;
    Ok(rendered)
}

fn shuffle_stories(stories: &mut Vec<Story>) {
    // Shuffle the stories in-place, as I don't want the same type of story up top each time.
    let mut rng = thread_rng();
    stories.shuffle(&mut rng);
}

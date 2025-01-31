use swanling::prelude::*;

use crate::common;

use rand::seq::SliceRandom;

/// Load the front page in Spanish and all static assets found on the page.
pub async fn front_page_es(user: &SwanlingUser) -> SwanlingTaskResult {
    let swanling = user.get("/es").await?;
    common::validate_and_load_static_assets(user, swanling, "Inicio").await?;

    Ok(())
}

/// Load article listing in Spanish and all static assets found on the page.
pub async fn recipe_listing_es(user: &SwanlingUser) -> SwanlingTaskResult {
    let swanling = user.get("/es/recipes/").await?;
    common::validate_and_load_static_assets(user, swanling, "Recetas").await?;

    Ok(())
}

/// Load a random recipe in Spanish and all static assets found on the page.
pub async fn recipe_es(user: &SwanlingUser) -> SwanlingTaskResult {
    let nodes = common::get_nodes(&common::ContentType::Recipe);
    let recipe = nodes.choose(&mut rand::thread_rng());
    let swanling = user.get(recipe.unwrap().url_es).await?;
    common::validate_and_load_static_assets(user, swanling, recipe.unwrap().title_es).await?;

    Ok(())
}

/// Load article listing in Spanish and all static assets found on the page.
pub async fn article_listing_es(user: &SwanlingUser) -> SwanlingTaskResult {
    let swanling = user.get("/es/articles/").await?;
    common::validate_and_load_static_assets(user, swanling, "Artículos").await?;

    Ok(())
}

/// Load a random article in Spanish and all static assets found on the page.
pub async fn article_es(user: &SwanlingUser) -> SwanlingTaskResult {
    let nodes = common::get_nodes(&common::ContentType::Article);
    let article = nodes.choose(&mut rand::thread_rng());
    let swanling = user.get(article.unwrap().url_es).await?;
    common::validate_and_load_static_assets(user, swanling, article.unwrap().title_es).await?;

    Ok(())
}

/// Load a basic page in Spanish and all static assets found on the page.
pub async fn basic_page_es(user: &SwanlingUser) -> SwanlingTaskResult {
    let nodes = common::get_nodes(&common::ContentType::BasicPage);
    let page = nodes.choose(&mut rand::thread_rng());
    let swanling = user.get(page.unwrap().url_es).await?;
    common::validate_and_load_static_assets(user, swanling, page.unwrap().title_es).await?;

    Ok(())
}

/// Anonymously load the contact form in Spanish and POST feedback.
pub async fn anonymous_contact_form_es(user: &SwanlingUser) -> SwanlingTaskResult {
    common::anonymous_contact_form(user, false).await?;

    Ok(())
}

// Pick a random word from the title of a random node and perform a search in Spanish.
pub async fn search_es(user: &SwanlingUser) -> SwanlingTaskResult {
    common::search(user, false).await?;

    Ok(())
}

/// Load category listing by a random term in Spanish and all static assets found on the page.
pub async fn term_listing_es(user: &SwanlingUser) -> SwanlingTaskResult {
    let terms = common::get_terms();
    let term = terms.choose(&mut rand::thread_rng());
    let swanling = user.get(term.unwrap().url_es).await?;
    common::validate_and_load_static_assets(user, swanling, term.unwrap().title_es).await?;

    Ok(())
}

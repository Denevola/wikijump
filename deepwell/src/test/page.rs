/*
 * test/page.rs
 *
 * DEEPWELL - Wikijump API provider and database manager
 * Copyright (C) 2019-2022 Wikijump Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use super::prelude::*;
use crate::services::page::{CreatePage, CreatePageOutput};

#[async_std::test]
async fn create() -> Result<()> {
    let app = setup().await?;

    let output: CreatePageOutput = app
        .post("/api/vI/page/1")
        .body(create_body(CreatePage {
            wikitext: str!("Page contents"),
            title: str!("Test page"),
            alt_title: None,
            slug: str!("test"),
            revision_comments: str!("Create page"),
            user_id: ADMIN_USER,
        }))
        .recv_json()
        .await
        .expect("Unable to send web request");

    println!("-- {:#?}", output);

    assert_eq!(&output.slug, "test", "Created page slug doesn't match");
    assert!(
        output.parser_warnings.is_empty(),
        "Parser warnings in page creation",
    );

    Ok(())
}

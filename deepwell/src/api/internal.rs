/*
 * api/internal.rs
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

//! Routes for the internal API.
//!
//! This version has no commitments to stability and is used only by Wikijump itself.

use crate::api::ApiServer;
use crate::methods::{
    category::*, file::*, file_revision::*, link::*, locales::*, misc::*, page::*,
    page_revision::*, parent::*, site::*, text::*, user::*, vote::*,
};
use crate::web::utils::error_response;
use tide::StatusCode;

pub fn build(mut app: ApiServer) -> ApiServer {
    // Miscellaneous
    app.at("/ping").all(ping);
    app.at("/version").get(version);
    app.at("/version/full").get(full_version);
    app.at("/ratelimit-exempt").all(ratelimit_exempt);
    app.at("/teapot")
        .get(|_| async { error_response(StatusCode::ImATeapot, "🫖") });

    // Localization
    app.at("/locale/:locale").head(locale_head).get(locale_get);

    app.at("/message/:locale/:message_key")
        .head(message_head)
        .get(message_post)
        .put(message_post)
        .post(message_post);

    // Site
    app.at("/site").post(site_create);
    app.at("/site/:type/:id_or_slug")
        .head(site_head)
        .get(site_get);

    // Category
    app.at("/category/:site_id").get(category_all_get);

    app.at("/category/direct/:category_id")
        .head(category_head_direct)
        .get(category_get_direct);

    app.at("/category/:site_id/:type/:id_or_slug")
        .head(category_head)
        .get(category_get);

    // Page
    app.at("/page/direct/:page_id")
        .head(page_head_direct)
        .get(page_get_direct);

    app.at("/page/:site_id").post(page_create);
    app.at("/page/:site_id/:type/:id_or_slug")
        .head(page_head)
        .get(page_get)
        .post(page_edit)
        .delete(page_delete);

    app.at("/page/:site_id/:type/:id_or_slug/move/:new_slug")
        .post(page_move);

    app.at("/page/:site_id/:page_id/rerender")
        .post(page_rerender);

    app.at("/page/:site_id/:page_id/restore").post(page_restore);

    // Page revisions
    app.at("/page/:site_id/:type/:id_or_slug/revision")
        .get(page_revision_info);

    app.at("/page/:site_id/:type/:id_or_slug/revision/:revision_number")
        .head(page_revision_head)
        .get(page_revision_get)
        .put(page_revision_put);

    app.at("/page/:site_id/:type/:id_or_slug/revision/:revision_number/rollback")
        .post(page_rollback);

    app.at("/page/:site_id/:type/:id_or_slug/revision/:revision_number/:direction")
        .get(page_revision_range_get);

    // Page links
    app.at("/page/:site_id/:type/:id_or_slug/links/from")
        .get(page_links_from_get);

    app.at("/page/:site_id/:type/:id_or_slug/links/to")
        .get(page_links_to_get);

    app.at("/page/:site_id/slug/:page_slug/links/to/missing")
        .get(page_links_to_missing_get);

    app.at("/page/:site_id/:type/:id_or_slug/urls")
        .get(page_links_external_from);

    app.at("/page/:site_id/urls/:url")
        .get(page_links_external_to);

    // Page parents
    app.at(
        "/page/:site_id/:parent_type/:parent_id_or_slug/:child_type/:child_id_or_slug",
    )
    .head(parent_head)
    .get(parent_get)
    .put(parent_put)
    .delete(parent_delete);

    app.at("/page/:site_id/:relationship_type/:type/:id_or_slug")
        .get(parent_relationships_get);

    // Page (invalid routes)
    app.at("/page").all(page_invalid);
    app.at("/page/:type/:id_or_slug").all(page_invalid);
    app.at("/page/:site_id/id/:page_slug/links/to/missing")
        .all(page_invalid);

    // Files
    app.at("/file/direct/:file_id")
        .head(file_head_direct)
        .get(file_get_direct);

    app.at("/file/:site_id/:type/:id_or_slug").post(file_create);

    app.at("/file/:site_id/:page_type/:id_or_slug/:file_type/:id_or_name")
        .head(file_head)
        .get(file_get)
        .post(file_edit)
        .delete(file_delete);

    app.at("/file/:site_id/:page_type/:id_or_slug/move")
        .post(file_move);

    app.at("/file/:site_id/:page_type/:id_or_slug/restore")
        .post(file_restore);

    // File revisions
    app.at("/file/:site_id/:page_type/:id_or_slug/:file_type/:id_or_name/revision")
        .get(file_revision_info);

    app.at("/file/:site_id/:page_type/:id_or_slug/:file_type/:id_or_name/revision/:revision_number")
        .head(file_revision_head)
        .get(file_revision_get)
        .put(file_revision_put);

    app.at("/file/:site_id/:page_type/:id_or_slug/:file_type/:id_or_name/revision/:revision_number/:direction")
        .get(file_revision_range_get);

    // Text
    // TEMP
    app.at("/text").put(text_put);
    app.at("/text/:hash").get(text_get).head(text_head);

    // User
    app.at("/user").post(user_create);
    app.at("/user/:type/:id_or_slug")
        .head(user_head)
        .get(user_get)
        .put(user_put)
        .delete(user_delete);

    // Votes
    app.at("/vote")
        .head(vote_head)
        .get(vote_get)
        .put(vote_put)
        .delete(vote_delete);

    app.at("/vote/direct/:vote_id")
        .head(vote_head_direct)
        .get(vote_get_direct);

    app.at("/vote/action").put(vote_action);
    app.at("/vote/list").get(vote_list_get);
    app.at("/vote/count").get(vote_count_get);

    app
}

/*
 * services/link/structs.rs
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

use crate::models::page_connection::Model as PageConnectionModel;
use crate::models::page_connection_missing::Model as PageConnectionMissingModel;
use crate::models::page_link::Model as PageLinkModel;
use sea_orm::entity::prelude::DateTimeWithTimeZone;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetLinksFromOutput {
    pub present: Vec<PageConnectionModel>,
    pub absent: Vec<PageConnectionMissingModel>,
    pub external: Vec<PageLinkModel>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetLinksToOutput {
    pub connections: Vec<PageConnectionModel>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetLinksToMissingOutput {
    pub connections: Vec<PageConnectionMissingModel>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetConnectionsFromOutput {
    pub present: Vec<PageConnectionModel>,
    pub absent: Vec<PageConnectionMissingModel>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetLinksExternalFromOutput {
    pub links: Vec<PageLinkModel>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetLinksExternalToOutput {
    pub links: Vec<ToExternalLink>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ToExternalLink {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub page_id: i64,
    pub count: i32,
}

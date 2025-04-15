use playground_dep::*;

#[macroe::model("DepModel", "DepEnum", "DepClass")]
pub struct _ExportModel {
    dep_enum: DepEnum,
    dep_model: DepModel,
    dep_class: DepClass,
}

use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;
#[repr(C)]
pub struct QModelIndex {
    _space: MaybeUninit<[usize; 3]>,
}

unsafe impl ExternType for QModelIndex {
    type Id = type_id!("QModelIndex");
    type Kind = cxx::kind::Trivial;
}

#[cxx_qt::bridge(cxx_file_stem = "minefield")]
mod minefield_bridge {
    use crate::data::{
        Data as MinefieldData,
        Status as TileStatus,
        Type as TileType,
    };

    unsafe extern "C++" {
        include!("QAbstractTableModelCxx.h");
        include!("QModelIndexCxx.h");
        include!("cxx-qt-lib/qvariant.h");
        
        type QVariant = cxx_qt_lib::QVariant;
        type QModelIndex = super::QModelIndex;
        
        fn row(self: &QModelIndex) -> i32;
        fn column(self: &QModelIndex) -> i32;

        #[cxx_name = "beginResetModel"]
        fn _begin_reset_model(self: Pin<&mut MinefieldQt>);
        #[cxx_name = "endResetModel"]
        fn _end_reset_model(self: Pin<&mut MinefieldQt>);
    }
    
    #[cxx_qt::qobject(base = "QAbstractTableModelCxx")]
    pub struct Minefield {
        minefield_data: MinefieldData,
    }
    
    impl std::default::Default for Minefield {
        fn default() -> Self {
            Minefield { minefield_data: MinefieldData::new(9, 12, 20) }
        }
    }
    
    impl qobject::Minefield {

        #[qinvokable(cxx_override)]
        fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
            match self.rust().minefield_data.index(index.row(), index.column()) {
                Some(tile) => match role {
                    0 => match tile.status {
                        TileStatus::Unknown => QVariant::from(0_i32),
                        TileStatus::Flagged => QVariant::from(1_i32),
                        TileStatus::Revealed => QVariant::from(2_i32),
                        TileStatus::Suspicious => QVariant::from(3_i32),
                    },
                    1 => match tile.ty {
                        TileType::Mine => QVariant::from(-1_i32),
                        TileType::Safe(v) => QVariant::from(v as i32),
                    },
                    _ => QVariant::default(),
                },
                None => QVariant::default(),
            }
        }

        #[qinvokable(cxx_override)]
        pub fn row_count(&self, _parent: &QModelIndex) -> i32 {
            self.rust().minefield_data.width()
        }
        
        #[qinvokable(cxx_override)]
        pub fn column_count(&self, _parent: &QModelIndex) -> i32 {
            self.rust().minefield_data.height()
        }
        
        #[qinvokable(cxx_override)]
        pub fn role_names_as_vec(&self) -> Vec<String> {
            vec!["status".to_owned(), "type".to_owned()]
        }
    }
}
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
        minefield_data: Vec<Vec<u32>>,
    }
    
    impl std::default::Default for Minefield {
        fn default() -> Self {
            Minefield {
                minefield_data: vec![
                    vec![1, 2, 3],
                    vec![4, 5, 6],
                ],
            }        
        }
    }
    
    impl qobject::Minefield {

        #[qinvokable(cxx_override)]
        fn data(&self, index: &QModelIndex, _role: i32) -> QVariant {
            QVariant::from(self.rust().minefield_data[index.row() as usize][index.column() as usize])
        }

        #[qinvokable(cxx_override)]
        pub fn row_count(&self, _parent: &QModelIndex) -> i32 {
            self.rust().minefield_data.len() as i32
        }
        
        #[qinvokable(cxx_override)]
        pub fn column_count(&self, _parent: &QModelIndex) -> i32 {
            self.rust().minefield_data[0].len() as i32
        }
    }
}
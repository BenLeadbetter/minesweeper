#pragma once

#include <QtCore/QAbstractTableModel>

class QAbstractTableModelCxx : public QAbstractTableModel
{
public:
    using QAbstractTableModel::QAbstractTableModel;
    
    // make these protected methods public so that
    // we can use them in our rust model
    using QAbstractTableModel::beginResetModel;
    using QAbstractTableModel::endResetModel;
};
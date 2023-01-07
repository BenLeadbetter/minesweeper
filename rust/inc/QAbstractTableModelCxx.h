#pragma once

#include <rust/cxx.h>

#include <QtCore/QAbstractTableModel>

class QAbstractTableModelCxx : public QAbstractTableModel
{
public:
    using QAbstractTableModel::QAbstractTableModel;
    using QAbstractTableModel::beginResetModel;
    using QAbstractTableModel::endResetModel;
    
    virtual rust::Vec<rust::String> roleNamesAsVec() const = 0;

    QHash<int, QByteArray> roleNames() const override
    {
        QHash<int, QByteArray> names;
        int i = 0;
        for (auto role : roleNamesAsVec()) {
          names.insert(i++, QByteArray::fromStdString((std::string)role));
        }
        return names;
    }
};
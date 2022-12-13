#pragma once

#include <rust/cxx.h>

#include <QtCore/QModelIndex>
#include <QtCore/QTypeInfo>

#include <type_traits>

template<> struct rust::IsRelocatable<QModelIndex> : std::true_type {};
static_assert(QTypeInfo<QModelIndex>::isRelocatable);
static_assert(alignof(QModelIndex) <= alignof(std::size_t[3]));
static_assert(sizeof(QModelIndex) == sizeof(std::size_t[3]));
import QtQuick
import QtQuick.Shapes

Item {
    id: tileBackground

    required property color color
    required property bool revealed

    Shape {
        anchors.fill: parent
        ShapePath {
            strokeWidth: -1
            fillColor: Qt.lighter(tileBackground.color, 1.5)
            PathLine { x: tile.width; y: tile.height }
            PathLine { x: tile.width; y: 0 }
            PathLine { x: 0; y: 0 }
        }
    }
    Shape {
        anchors.fill: parent
        ShapePath {
            strokeWidth: -1
            fillColor: Qt.darker(tileBackground.color, 1.5)
            PathLine { x: 0; y: tile.height }
            PathLine { x: tile.width; y: tile.height }
            PathLine { x: 0; y: 0 }
        }
    }
    Rectangle {
        anchors{
            fill: parent
            margins: tile.padding
        }
        color: tileBackground.color
    }
}
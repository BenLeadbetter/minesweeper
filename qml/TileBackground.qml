import QtQuick
import QtQuick.Shapes

Item {
    id: tileBackground

    required property color color
    required property int status

    Rectangle {
        anchors.fill: parent
        visible: tileBackground.status === Tile.Revealed
        color: tileBackground.color
    }
    Shape {
        anchors.fill: parent
        visible: tileBackground.status !== Tile.Revealed
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
        visible: tileBackground.status !== Tile.Revealed
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
        visible: tileBackground.status !== Tile.Revealed
        color: tileBackground.color
    }
}
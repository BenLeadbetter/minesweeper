import QtQuick
import QtQuick.Templates as T

T.Control {
    id: tile

    property string text

    implicitWidth: 30
    implicitHeight: 30
    padding: 3
    hoverEnabled: true

    contentItem: Item {
        Text {
            text: tile.text
            anchors.fill: parent
            horizontalAlignment: Text.AlignHCenter
            verticalAlignment: Text.AlignVCenter
        }
    }     
    background: TileBackground {
        id: bg
        color: d.defaultBackgroundBaseColor
    }
    states: [
        State {
            when: tile.hovered
            PropertyChanges {
                bg.color: Qt.lighter(d.defaultBackgroundBaseColor, 1.2)
            }
        }
    ]
    
    QtObject {
        id: d
        readonly property color defaultBackgroundBaseColor: "#969696"
    }
    
}
import QtQuick
import QtQuick.Templates as T

T.Control {
    id: tile

    enum Status {
        Unknown,
        Flagged,
        Revealed,
        Suspicious
    }

    property int status
    property int adjacentMines
    property bool mine
    
    signal reveal
    signal mark
    
    implicitWidth: 30
    implicitHeight: 30
    padding: 3
    hoverEnabled: true

    contentItem: Item {
        Text {
            text: tile.adjacentMines
            anchors.fill: parent
            horizontalAlignment: Text.AlignHCenter
            verticalAlignment: Text.AlignVCenter
        }
    }     
    background: TileBackground {
        id: bg
        color: d.defaultBackgroundBaseColor
        status: tile.status

        TapHandler {
            acceptedButtons: Qt.LeftButton
            onTapped: {
                if (tile.status !== Tile.Revealed) {
                    tile.reveal()
                }
            }
        }

        TapHandler {
            acceptedButtons: Qt.RightButton
            onTapped: {
                if (tile.status !== Tile.Revealed) {
                    tile.mark()
                }
            }
        }
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
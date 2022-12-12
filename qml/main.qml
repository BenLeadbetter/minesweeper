import QtQuick
import QtQuick.Controls

import Minesweeper

ApplicationWindow {
    width: 640
    height: 480

    title: qsTr("Minesweeper")
    color: "hotpink"
    visible: true
    
    TableView {
        id: minefield
        anchors.fill: parent
        columnSpacing: 10
        rowSpacing: 10
        model: Minefield {}        
        delegate: Rectangle {
            implicitWidth: 100
            implicitHeight: 100
            color: "lightblue"
            Text {
                text: type
                anchors.fill: parent
                horizontalAlignment: Text.AlignHCenter
                verticalAlignment: Text.AlignVCenter
            }
        }
    }
}


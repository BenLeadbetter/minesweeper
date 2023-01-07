import QtQuick
import QtQuick.Controls

import Minesweeper

ApplicationWindow {
    width: minefield.contentWidth
    height: minefield.contentHeight
    maximumWidth: minefield.contentWidth
    maximumHeight: minefield.contentHeight
    minimumWidth: minefield.contentWidth
    minimumHeight: minefield.contentHeight

    title: qsTr("Minesweeper")
    color: "hotpink"
    visible: true
    
    TableView {
        id: minefield
        anchors.fill: parent
        columnSpacing: 0
        rowSpacing: 0
        model: Minefield {}        
        delegate: Tile {
            text: type
        }
    }
}


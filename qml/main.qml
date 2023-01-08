import QtQuick
import QtQuick.Controls

import Minesweeper

ApplicationWindow {
    width: 30 * minefieldModel.cols()
    height: 30 * minefieldModel.rows()
    minimumHeight: height
    maximumHeight: height
    minimumWidth: width
    maximumWidth: width

    title: qsTr("Minesweeper")
    color: "hotpink"
    visible: true
    
    TableView {
        id: minefield
        anchors.fill: parent
        columnSpacing: 0
        rowSpacing: 0
        model: Minefield {
            id: minefieldModel
        }
        delegate: Tile {
            adjacentMines: model.adjacentMines
            mine: model.mine
            status: model.status
            onReveal: minefieldModel.reveal(index);
            onMark: minefieldModel.mark(index);
        }
    }
}


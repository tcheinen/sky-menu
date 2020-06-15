import QtQuick 2.6
import QtQuick.Window 2.2

import Launcher 1.0
Window {
    id: window
    visible: launcher.visible
    width: launcher.width
    height: launcher.height
    flags: Qt.Dialog | Qt.FramelessWindowHint
    color: "#00000000"

    Launcher {
        id: launcher
    }

    onVisibleChanged: {
        if (visible) {
            raise()
            query_input.text = ""
        }
    }

    onActiveChanged: {
        if (!active) {
            launcher.hide()
        }
    }

    Component.onCompleted: {
        launcher.setup()
        setX(Screen.width / 2 - width / 2)
        setY(Screen.height / 2 - height / 2)
    }

    Item {
        anchors.fill: parent
        Rectangle {
            id: background
            color: "#00153D"
            border.width: 0
            opacity: 0.5
            height: launcher.height * (launcher.model_len + 1) * .1
            width: launcher.width
        }

        Rectangle {
            height: launcher.height * .1
            color: "#D3F8E2"
            anchors.left: parent.left
            anchors.right: parent.right

            TextInput {
                id: query_input
                color: "#000000"
                anchors.leftMargin: 30
                horizontalAlignment: Text.AlignLeft
                font.pointSize: 16
                renderType: Text.QtRendering
                cursorVisible: true
                font.family: "Iosevka Aile"
                verticalAlignment: Text.AlignVCenter
                anchors.fill: parent
                focus: true
                onTextChanged: launcher.search(text)
                Keys.onUpPressed: launcher.up()
                Keys.onDownPressed: launcher.down()
                Keys.onReturnPressed: launcher.launch()
                Keys.onEscapePressed: launcher.hide()
            }
        }

        ListView {
            anchors.bottom: parent.bottom
            anchors.right: parent.right
            anchors.left: parent.left
            height: launcher.height * .9
            currentIndex: launcher.selected
            highlightMoveDuration: 0
            model: launcher.model
            highlight: Rectangle {
                color: "#BA9BFF"
            }
            delegate: Item {
                id: listItem
                height: launcher.height * 0.1
                width: launcher.width
                Image {
                    height: launcher.height * 0.1
                    width: launcher.height * 0.1
                    source: launcher.icon(icon)
                }
                Text {
                    leftPadding: launcher.height * 0.1
                    text: name
                    anchors.fill: parent
                    font.family: "Iosevka Aile"
                    verticalAlignment: Text.AlignVCenter
                    font.pointSize: 16
                    color: listItem.ListView.isCurrentItem ? "black" : "white"
                }
            }
        }
    }
}

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

    Component.onCompleted: {
        launcher.setup()
        setX(Screen.width / 2 - width / 2)
        setY(Screen.height / 2 - height / 2)
    }

    Rectangle {
        id: rectangle
        color: "#808080"
        anchors.fill: parent

        Rectangle {
            height: window.height * .1
            color: "#ffffff"
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
                font.family: "Times New Roman"
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
            height: window.height * .9
            currentIndex: launcher.selected
            model: launcher.model
            highlight: Rectangle {
                color: "steelblue"

            }
            delegate: Item {
                height: window.height * 0.1
                width: window.width
                Image {
                    height: window.height * 0.1
                    width: window.height * 0.1
                    source: launcher.icon(icon)
                }
                Text {
                    leftPadding: window.height * 0.1
                    text: name
                    color: "white"
                }
            }
        }
    }
}

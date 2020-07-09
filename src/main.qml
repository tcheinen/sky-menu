import QtQuick 2.6
import QtQuick.Window 2.2

import Launcher 1.0
import Config 1.0
Window {
    id: window
    visible: launcher.visible
    width: config.width
    height: config.height
    flags: Qt.Dialog | Qt.FramelessWindowHint
    color: "#00000000"
    title: "Launcher"

    SystemPalette { id: palette; colorGroup: SystemPalette.Active }

    Launcher {
        id: launcher
    }

    Shortcut {
        sequence: "Tab"
        onActivated: launcher.down()
    }
    Shortcut {
        sequence: "Alt+Tab"
        onActivated: launcher.down()
    }
    Shortcut {
        sequence: ["Shift+Tab", "Alt+Shift+Tab"]
        onActivated: launcher.up()
    }
    Shortcut {
        sequence: "Alt+Shift+Tab"
        onActivated: launcher.up()
    }

    Config {
        id: config
    }

    onVisibleChanged: {
        if (visible) {
            raise()
            query_input.text = ""
        }
    }

    onActiveChanged: {
        if (!active) {
            launcher.hide_if_launcher()
        }
    }

    Component.onCompleted: {
        launcher.setup()
        config.setup()
        setX(Screen.width / 2 - width / 2)
        setY(Screen.height / 2 - height / 2)
    }

    Item {
        anchors.fill: parent
        Rectangle {
            id: background
            color: palette.window
            border.width: 0
            opacity: 0.5
            height: config.height * (launcher.model_len + 1) * .1
            width: config.width
        }

        Rectangle {
            height: config.height * .1
            color: palette.base
            anchors.left: parent.left
            anchors.right: parent.right

            TextInput {
                id: query_input
                color:  palette.text
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
            height: config.height * .9
            currentIndex: launcher.selected
            highlightMoveDuration: 0
            model: launcher.model
            highlight: Rectangle {
                color: palette.highlight
            }
            delegate: Item {
                id: listItem
                height: config.height * 0.1
                width: config.width
                Image {
                    height: config.height * 0.1
                    width: config.height * 0.1
                    source: launcher.icon(icon)
                }
                Text {
                    leftPadding: config.height * 0.1
                    text: name
                    anchors.fill: parent
                    font.family: "Iosevka Aile"
                    verticalAlignment: Text.AlignVCenter
                    font.pointSize: 16
                    color: listItem.ListView.isCurrentItem ? palette.highlightedText : palette.windowText
                }
            }
        }
    }
}

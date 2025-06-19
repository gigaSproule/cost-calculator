package com.benjaminsproule.costcalculator

import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.unit.DpSize
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.ApplicationScope
import androidx.compose.ui.window.Window
import androidx.compose.ui.window.WindowPosition
import androidx.compose.ui.window.WindowState
import androidx.compose.ui.window.application
import com.benjaminsproule.costcalculator.ui.Content
import java.awt.Dimension
import java.awt.Toolkit

fun main() = application {
    Viewer()
}

@Composable
fun ApplicationScope.Viewer() {
    Window(
        onCloseRequest = ::exitApplication,
        title = "Desktop Viewer",
        state = WindowState(
            position = WindowPosition.Aligned(Alignment.Center),
            size = getPreferredWindowSize(720, 857)
        ),
//        icon = painterResource(Res.drawable.ic_imageviewer_round),
        // https://github.com/JetBrains/compose-jb/issues/2741
//        onKeyEvent = {
//            if (it.type == KeyEventType.KeyUp) {
//                when (it.key) {
//                    Key.DirectionLeft -> externalNavigationEventBus.produceEvent(
//                        ExternalImageViewerEvent.Previous
//                    )
//
//                    Key.DirectionRight -> externalNavigationEventBus.produceEvent(
//                        ExternalImageViewerEvent.Next
//                    )
//
//                    Key.Escape -> externalNavigationEventBus.produceEvent(
//                        ExternalImageViewerEvent.ReturnBack
//                    )
//                }
//            }
//            false
//        }
    ) {
        Content()
    }
}

private fun getPreferredWindowSize(desiredWidth: Int, desiredHeight: Int): DpSize {
    val screenSize: Dimension = Toolkit.getDefaultToolkit().screenSize
    val preferredWidth: Int = (screenSize.width * 0.8f).toInt()
    val preferredHeight: Int = (screenSize.height * 0.8f).toInt()
    val width: Int = if (desiredWidth < preferredWidth) desiredWidth else preferredWidth
    val height: Int = if (desiredHeight < preferredHeight) desiredHeight else preferredHeight
    return DpSize(width.dp, height.dp)
}
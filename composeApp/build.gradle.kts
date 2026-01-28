import org.gradle.kotlin.dsl.invoke
import org.jetbrains.compose.desktop.application.dsl.TargetFormat
import org.jetbrains.kotlin.gradle.dsl.JvmTarget

plugins {
    alias(libs.plugins.kotlin.multiplatform)
//    alias(libs.plugins.kotlin.android)
    alias(libs.plugins.android.multiplatform.library)
    alias(libs.plugins.compose)
    alias(libs.plugins.compose.compiler)
    id(libs.plugins.compose.hot.reload.get().pluginId)
    alias(libs.plugins.kotlin.serialization)
}

version = "1.0-SNAPSHOT"

kotlin {

    androidLibrary {
        namespace = "com.benjaminsproule.costcalculatorlibrary"
        compileSdk = 36

        compilerOptions {
            jvmTarget.set(JvmTarget.JVM_25)
        }

        androidResources {
            enable = true
        }
    }

//    listOf(
//        iosX64(),
//        iosArm64(),
//        iosSimulatorArm64()
//    ).forEach { iosTarget ->
//        iosTarget.binaries.framework {
//            baseName = "composeApp"
//            isStatic = true
//        }
//    }

//    js {
//        browser()
//        useEsModules()
//    }

    jvm("desktop")

//    wasmJs { browser() }

    sourceSets {

        commonMain.dependencies {
            implementation(compose.runtime)
            implementation(compose.material3)
            implementation(compose.materialIconsExtended)
            implementation(compose.ui)
            implementation(compose.components.resources)
            implementation(compose.components.uiToolingPreview)
            implementation(libs.kotlinx.serialization.json)
            implementation(libs.compose.lifecycle.viewmodel.compose)
            implementation(libs.jetbrains.androidx.navigation.compose)
        }

        androidMain.dependencies {
            implementation(libs.androidx.datastore)
            implementation(libs.androidx.datastore.preferences)
        }

        val desktopMain by getting
        desktopMain.dependencies {
            implementation(compose.desktop.currentOs)
            implementation(libs.kotlinx.coroutines.swing)
            implementation(libs.ktoml.core)
        }

//        val jsWasmMain by creating {
//            dependsOn(commonMain.get())
//            dependencies {
//                implementation(npm("uuid", "^9.0.1"))
//            }
//        }
//
//        val jsMain by getting {
//            dependsOn(jsWasmMain)
//        }
//
//        val wasmJsMain by getting {
//            dependsOn(jsWasmMain)
//        }

//        val desktopMain by getting
//        desktopMain.dependencies {
//            implementation(compose.desktop.common)
//        }
//        val desktopTest by getting
//        desktopTest.dependencies {
//            implementation(compose.desktop.currentOs)
//            implementation(compose.desktop.uiTestJUnit4)
//        }
    }

    dependencies {
//        implementation(projects.composeApp)
        implementation(libs.compose.ui.tooling.preview)
    }
}

dependencies {
    androidRuntimeClasspath(compose.uiTooling)
}

compose.desktop {
    application {
        mainClass = "com.benjaminsproule.costcalculator.MainKt"

        nativeDistributions {
            targetFormats(TargetFormat.Dmg, TargetFormat.Msi, TargetFormat.Deb)
            packageName = "com.benjaminsproule.costcalculator"
            packageVersion = "1.0.0"

            val iconsRoot = project.file("desktop-icons")
            macOS {
                iconFile.set(iconsRoot.resolve("icon-mac.icns"))
            }
            windows {
                iconFile.set(iconsRoot.resolve("icon-windows.ico"))
                menuGroup = "Compose Examples"
                // see https://wixtoolset.org/documentation/manual/v3/howtos/general/generate_guids.html
                upgradeUuid = "18159995-d967-4CD2-8885-77BFA97CFA9F"
            }
            linux {
                iconFile.set(iconsRoot.resolve("icon-linux.png"))
            }
        }
    }

    kotlin {
        jvmToolchain(25)
    }
}

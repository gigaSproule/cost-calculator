import org.jetbrains.compose.desktop.application.dsl.TargetFormat

plugins {
    alias(libs.plugins.kotlin.multiplatform)
    alias(libs.plugins.android.application)
    alias(libs.plugins.compose)
    alias(libs.plugins.compose.compiler)
    alias(libs.plugins.compose.hot.reload)
    alias(libs.plugins.kotlin.serialization)
}

version = "1.0-SNAPSHOT"

kotlin {
    androidTarget()

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
            implementation(compose.preview)
            implementation(libs.androidx.activity.compose)
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
}

android {
    compileSdk = 36
    namespace = "com.benjaminsproule.costcalculator"

    defaultConfig {
        applicationId = "com.benjaminsproule.costcalculator"
        minSdk = 29
        targetSdk = 36
        versionCode = 7
        versionName = "0.0.3"

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
        vectorDrawables {
            useSupportLibrary = true
        }
    }
    lint {
        targetSdk = 36
    }
    buildTypes {
        release {
            isMinifyEnabled = true
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "androidMain/proguard-rules.pro"
            )
            ndk {
                debugSymbolLevel = "FULL"
            }
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_21
        targetCompatibility = JavaVersion.VERSION_21
    }
    kotlin {
        jvmToolchain(21)
    }
    buildFeatures {
        compose = true
    }
    packaging {
        resources {
            excludes += "/META-INF/{AL2.0,LGPL2.1}"
        }
    }
}

dependencies {
    debugImplementation(compose.uiTooling)
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
        jvmToolchain(21)
    }
}
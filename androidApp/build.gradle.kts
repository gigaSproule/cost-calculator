import org.jetbrains.kotlin.gradle.dsl.JvmTarget

plugins {
    alias(libs.plugins.android.application)
//    alias(libs.plugins.compose.multiplatform)
    alias(libs.plugins.compose.compiler)
}

kotlin {
    dependencies {
        implementation(project(":composeApp"))
        implementation(libs.androidx.material3)
        implementation(libs.androidx.activity.compose)
        implementation(libs.androidx.datastore)
        implementation(libs.androidx.datastore.preferences)
        implementation(libs.androidx.ui.tooling.preview)
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
        sourceCompatibility = JavaVersion.VERSION_25
        targetCompatibility = JavaVersion.VERSION_25
    }
    kotlin {
        jvmToolchain(25)
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

//target {
//    compilerOptions {
//        jvmTarget.set(JvmTarget.JVM_25)
//    }
//}

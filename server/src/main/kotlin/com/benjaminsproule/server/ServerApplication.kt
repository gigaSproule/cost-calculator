package com.benjaminsproule.server

import org.springframework.boot.actuate.autoconfigure.security.reactive.ReactiveManagementWebSecurityAutoConfiguration
import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.autoconfigure.security.reactive.ReactiveSecurityAutoConfiguration
import org.springframework.boot.runApplication

@SpringBootApplication(
    exclude = [ReactiveSecurityAutoConfiguration::class, ReactiveManagementWebSecurityAutoConfiguration::class]
)
class ServerApplication

fun main(args: Array<String>) {
    runApplication<ServerApplication>(*args)
}

/**
 * TODO:
 *
 * POST /sale
 * GET /sale
 * DELETE /sale
 *
 * POST /expenditure
 * GET /expenditure
 * DELETE /expenditure
 */

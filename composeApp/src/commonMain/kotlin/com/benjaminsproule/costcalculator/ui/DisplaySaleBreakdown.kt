package com.benjaminsproule.costcalculator.ui

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.benjaminsproule.costcalculator.calculator.SaleBreakdown
import com.benjaminsproule.costcalculator.store.Config

@Composable
fun DisplaySaleBreakdown(
    saleBreakdown: SaleBreakdown?,
    config: Config
) {
    LazyVerticalGrid(
        columns = GridCells.Fixed(2),
        verticalArrangement = Arrangement.Center
    ) {
        item {
            Text(
                modifier = Modifier.padding(end = 8.dp),
                text = "Sale"
            )
        }
        item {
            Text(
                if (saleBreakdown == null) {
                    ""
                } else {
                    "${config.currency}%.2f".format(saleBreakdown!!.sale)
                }
            )
        }
        item {
            Text(
                modifier = Modifier.padding(end = 8.dp), text = "Delivery costs"
            )
        }
        item {
            Text(
                if (saleBreakdown == null) {
                    ""
                } else {
                    "${config.currency}%.2f".format(saleBreakdown!!.deliveryCosts)
                }
            )
        }
        item {
            Text(modifier = Modifier.padding(end = 8.dp), text = "Transaction cost")
        }
        item {
            Text(
                if (saleBreakdown == null) {
                    ""
                } else {
                    "${config.currency}%.2f".format(saleBreakdown!!.transactionCost)
                }
            )
        }
        item {
            Text(
                modifier = Modifier.padding(end = 8.dp),
                text = "Payment processing cost"
            )
        }
        item {
            Text(
                if (saleBreakdown == null) {
                    ""
                } else {
                    "${config.currency}%.2f".format(saleBreakdown!!.paymentProcessingCost)
                }
            )
        }
        item {
            Text(modifier = Modifier.padding(end = 8.dp), text = "Offsite ads cost")
        }
        item {
            Text(
                if (saleBreakdown == null) {
                    ""
                } else {
                    "${config.currency}%.2f".format(saleBreakdown!!.offsiteAdsCost)
                }
            )
        }
        item {
            Text(modifier = Modifier.padding(end = 8.dp), text = "Regulatory operating fee")
        }
        item {
            Text(
                if (saleBreakdown == null) {
                    ""
                } else {
                    "${config.currency}%.2f".format(saleBreakdown!!.regulatoryOperatingFee)
                }
            )
        }
        item {
            Text(modifier = Modifier.padding(end = 8.dp), text = "VAT paid by buyer")
        }
        item {
            Text(
                if (saleBreakdown == null) {
                    ""
                } else {
                    "${config.currency}%.2f".format(saleBreakdown!!.vatPaidByBuyer)
                }
            )
        }
        item {
            Text(modifier = Modifier.padding(end = 8.dp), text = "VAT on seller fees")
        }
        item {
            Text(
                if (saleBreakdown == null) {
                    ""
                } else {
                    "${config.currency}%.2f".format(saleBreakdown!!.vatOnSellerFees)
                }
            )
        }
        item {
            Text(modifier = Modifier.padding(end = 8.dp), text = "Total fees")
        }
        item {
            Text(
                if (saleBreakdown == null) {
                    ""
                } else {
                    "${config.currency}%.2f".format(saleBreakdown!!.totalFees)
                }
            )
        }
        item {
            Text(modifier = Modifier.padding(end = 8.dp), text = "Total fees with VAT")
        }
        item {
            Text(
                if (saleBreakdown == null) {
                    ""
                } else {
                    "${config.currency}%.2f".format(saleBreakdown!!.totalFeesWithVat)
                }
            )
        }
        item {
            Text(modifier = Modifier.padding(end = 8.dp), text = "Tax")
        }
        item {
            Text(
                if (saleBreakdown == null) {
                    ""
                } else {
                    "${config.currency}%.2f".format(saleBreakdown!!.tax)
                }
            )
        }
        item {
            Text(modifier = Modifier.padding(end = 8.dp), text = "Revenue")
        }
        item {
            Text(
                if (saleBreakdown == null) {
                    ""
                } else {
                    "${config.currency}%.2f".format(saleBreakdown!!.revenue)
                }
            )
        }
        item {
            Text(modifier = Modifier.padding(end = 8.dp), text = "Percentage kept")
        }
        item {
            Text(
                if (saleBreakdown == null) {
                    ""
                } else {
                    "%.2f%%".format(saleBreakdown!!.percentageKept)
                }
            )
        }
        item {
            Text(modifier = Modifier.padding(end = 8.dp), text = "Max working hours")
        }
        item {
            Text(
                if (saleBreakdown == null) {
                    ""
                } else {
                    "%d:%02d".format(
                        saleBreakdown!!.maxWorkingHours.toInt(),
                        ((saleBreakdown!!.maxWorkingHours - (saleBreakdown!!.maxWorkingHours.toInt())) * 60).toInt()
                    )
                }
            )
        }
    }
}

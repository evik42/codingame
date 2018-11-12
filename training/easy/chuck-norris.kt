import java.util.*
import kotlin.streams.asSequence

fun main(args : Array<String>) {
    val input = Scanner(System.`in`)
    val (res, _) = input.nextLine().chars().flatMap { String.format("%7s", Integer.toBinaryString(it)).replace(' ', '0').chars() } .asSequence().fold(StringBuilder() to -1) {
        (sb, last), i ->
        if (last != i) {
            if (last >= 0 ) sb.append(" ")
            sb.append(if (i == '0'.toInt()) "00" else "0").append(" ")
        }
        sb.append("0")
        sb to i
    }

    println(res.toString())
}

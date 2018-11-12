import java.util.*
import kotlin.streams.toList

fun main(args : Array<String>) {
    val input = Scanner(System.`in`)
    val l = input.nextInt()
    val h = input.nextInt()
    input.nextLine()

    val message = input.nextLine()
        .toUpperCase()
        .chars()
        .map { if (it < 'A'.toInt() || it > 'Z'.toInt()) 26 else it - 'A'.toInt() }
        .toList()
        .map { it * l to it * l + l }

    (0 until h).forEach {
        val row = input.nextLine()
        message.forEach { (start, end) -> print(row.substring(start, end)) }
        println()
    }
}

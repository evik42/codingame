import java.util.*

fun main(args : Array<String>) {
    val input = Scanner(System.`in`)
    val res = input.nextLine()
        .split(' ')
        .windowed(2, 2)
        .flatMap { p -> List(p[1].length) { if (p[0] == "0") "1" else "0" } }
        .windowed(7, 7)
        .map { Integer.parseInt(it.joinToString(separator = ""), 2).toChar() }
        .joinToString(separator = "")

    println(res)
}

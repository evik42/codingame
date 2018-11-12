import java.util.*

fun main(args : Array<String>) {
    val input = Scanner(System.`in`)

    while (true) {
        println((0 until 8).map { it to input.nextInt() } .maxBy { it.second } ?.first)
    }
}

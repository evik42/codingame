import java.util.*

fun main(args : Array<String>) {
    val input = Scanner(System.`in`)
    val N = input.nextInt()
    println((0 until N).map {input.nextInt() } .sorted().windowed(2).map { l -> l[1] - l[0] } .min())
}

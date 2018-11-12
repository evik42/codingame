import java.lang.Math.abs
import java.lang.Math.max
import java.util.*

fun main(args : Array<String>) {
    val input = Scanner(System.`in`)
    val n = input.nextInt() // the number of temperatures to analyse
    val res = if (n == 0) 0 else (0 until n).fold(Int.MAX_VALUE) {
        acc, _ ->
        val t = input.nextInt()
        if (abs(t) < abs(acc)) t else if (abs(t) == abs(acc)) max(t, acc) else acc
    }

    println(res)
}

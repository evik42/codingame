import java.lang.Math.floor
import java.util.*

data class Light(val distance: Int, val duration: Int)

fun main(args : Array<String>) {
    val input = Scanner(System.`in`)
    val speedLimit = input.nextInt()
    val lights = (0 until input.nextInt()).map { Light(floor(input.nextDouble() * 3.6).toInt(), input.nextInt()) }

    tailrec fun calculate(speed: Int): Int = if (lights.fold(true) { r, l -> if (r) ((l.distance / speed) / l.duration) % 2 == 0 else r }) speed else calculate(speed - 1)

    println(calculate(speedLimit))
}

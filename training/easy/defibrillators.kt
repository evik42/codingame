import java.lang.Math.PI
import java.lang.Math.cos
import java.util.*

fun Double.toRad() = this * PI / 180
fun String.toFixedDouble() = this.replace(',', '.').toDouble()

data class Coord(val lon: Double, val lat: Double) {
    fun distance(other: Coord): Double {
        val x = (lon - other.lon) * cos((lat.toRad() + other.lat.toRad()) / 2)
        val y = lat - other.lat
        return x * x + y * y
    }
}


fun main(args : Array<String>) {
    val input = Scanner(System.`in`)
    val user = Coord(input.next().toFixedDouble(), input.next().toFixedDouble())
    val n = input.nextInt()
    input.nextLine()

    val (res, _) = (0 until n).fold("" to Double.MAX_VALUE) {
            (name, d), _ ->
        val def = input.nextLine().split(';')
        val dist = user.distance(Coord(def[4].toFixedDouble(), def[5].toFixedDouble()))
        if (dist < d) def[1] to dist else name to d
    }

    println(res)
}

import java.lang.Math.ceil
import java.util.*

fun main(args : Array<String>) {
    val input = Scanner(System.`in`)
    val surface =  (0 until input.nextInt()).map { input.nextInt(); input.nextInt() } .windowed(2).fold(-1) {
        acc, list ->
        if (list[0] == list[1]) list[0] else acc
    }

    while (true) {
        input.nextInt()
        val y = input.nextInt()
        input.nextInt()
        val vSpeed = input.nextInt()
        input.nextInt()
        input.nextInt()
        val power = input.nextInt()

        print("0 ")
        if (power > 0) {
            println(if (vSpeed < -39) 4 else 3)
        } else {
            val s = - vSpeed + 3.711
            val road = y - surface - (- vSpeed)
            val burn = s * 4 + 2.711 * 3 + 1.711 * 2 + 0.711
            val left = road - burn
            val t = ceil((s - 34.867) / 0.299)
            println(if (t > 0 && s * t + 0.1495 * t * t > left) 4 else 0)
        }
    }
}

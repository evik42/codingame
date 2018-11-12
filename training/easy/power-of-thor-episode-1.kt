import java.util.*

fun main(args : Array<String>) {
    val input = Scanner(System.`in`)
    val lightX = input.nextInt() // the X position of the light of power
    val lightY = input.nextInt() // the Y position of the light of power
    val lat = listOf("N", "", "S")
    val lon = listOf("W", "", "E")

    tailrec fun move(x: Int, y: Int) {
        val yOff = lightY.compareTo(y)
        print(lat[yOff + 1])
        val xOff = lightX.compareTo(x)
        print(lon[xOff + 1])
        println()
        move(x + xOff, y + yOff)
    }

    move(input.nextInt(), input.nextInt())
}

import java.util.*

fun main(args : Array<String>) {
    val input = Scanner(System.`in`)
    val n = input.nextInt()
    val graph = mutableMapOf<Int, MutableSet<Int>>().apply {
        (0 until n).forEach {
            val xi = input.nextInt() // the ID of a person which is adjacent to yi
            val yi = input.nextInt() // the ID of a person which is adjacent to xi
            getOrPut(xi) { mutableSetOf() } .add(yi)
            getOrPut(yi) { mutableSetOf() } .add(xi)
            Unit
        }
    }

    var d = 0
    while (graph.size > 1) {
        d += 1
        val leafs = graph.filter { (_, v) -> v.size <= 1 }
        graph.apply {
            leafs.forEach {
                k, v ->
                remove(k)
                if (v.isNotEmpty()) getValue(v.first()).remove(k)
            }
        }
    }
    println(d)
}

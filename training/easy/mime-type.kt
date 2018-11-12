import java.util.Scanner

fun main(args : Array<String>) {
    val input = Scanner(System.`in`)
    val n = input.nextInt() // Number of elements which make up the association table.
    val q = input.nextInt() // Number Q of file names to be analyzed.
    val mimeTypes = (0 until n).associate {
        input.next().toLowerCase() to input.next()
    }
    input.nextLine()
    (0 until q).map {
        val fname = input.nextLine() // One file name per line.
        val ext = fname.substring(fname.lastIndexOf('.') + 1).toLowerCase()
        if (ext.length < fname.length) mimeTypes.getOrDefault(ext,"UNKNOWN") else "UNKNOWN"
    } .forEach(::println)
}

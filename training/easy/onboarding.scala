import scala.io.StdIn._

object Onboarding extends App {
  while(true) {
    val enemy1 = readLine // name of enemy 1
    val dist1 = readInt // distance to enemy 1
    val enemy2 = readLine // name of enemy 2
    val dist2 = readInt // distance to enemy 2

    println(if (dist1 < dist2) enemy1 else enemy2) // You have to output a correct ship name to shoot ("Buzz", enemy1, enemy2, ...)
  }
}

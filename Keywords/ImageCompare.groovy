import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.util.KeywordUtil
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows

import internal.GlobalVariable
import java.awt.image.BufferedImage
import java.nio.file.Files
import java.nio.file.Path
import java.nio.file.Paths
import java.time.ZonedDateTime
import java.time.format.DateTimeFormatter

import javax.imageio.ImageIO

import org.openqa.selenium.WebDriver
import org.openqa.selenium.WebElement
import com.kms.katalon.core.configuration.RunConfiguration

import com.kms.katalon.core.webui.driver.DriverFactory

import com.kms.katalon.core.webui.common.WebUiCommonHelper
import ru.yandex.qatools.ashot.AShot
import ru.yandex.qatools.ashot.Screenshot
import ru.yandex.qatools.ashot.coordinates.WebDriverCoordsProvider
import ru.yandex.qatools.ashot.shooting.ShootingStrategies
import ru.yandex.qatools.ashot.comparison.ImageDiff
import ru.yandex.qatools.ashot.comparison.ImageDiffer

import java.awt.Image
import java.awt.Toolkit
import java.awt.Color
import java.awt.image.PixelGrabber

import org.openqa.selenium.By;
import com.kms.katalon.core.logging.KeywordLogger

public class test {

	/**
	 * Porovna dva obrazky (BufferedImage) s povolenou toleranciou percenta rozdielnych pixelov.
	 */
	boolean areImagesSimilar(BufferedImage img1, BufferedImage img2, double tolerancePercent) {
		if (img1.getWidth() != img2.getWidth() || img1.getHeight() != img2.getHeight()) {
			return false
		}
		int width = img1.getWidth()
		int height = img1.getHeight()
		int totalPixels = width * height
		int diffPixels = 0

		for (int y = 0; y < height; y++) {
			for (int x = 0; x < width; x++) {
				if (img1.getRGB(x, y) != img2.getRGB(x, y)) {
					diffPixels++
				}
			}
		}
		double percentDiff = (diffPixels * 100.0) / totalPixels
		println("Porovnanie difference: " + percentDiff + " % rozdielnych pixelov")
		return percentDiff <= tolerancePercent
	}

	@Keyword
	def compareImagesWithOptionalSecond(String pathExpected, String pathActual1, String pathActual2 = null, String outputName) {
		BufferedImage expected = ImageIO.read(new File(pathExpected))
		BufferedImage actual1 = ImageIO.read(new File(pathActual1))
		ImageDiffer differ = new ImageDiffer()
		def filePath = RunConfiguration.getProjectDir()
		double tolerance = 25 // povolené percento rozdielnych pixelov

		if (pathActual2 == null || pathActual2.trim().isEmpty()) {
			boolean similar1 = areImagesSimilar(expected, actual1, tolerance)
			ImageDiff diff1 = differ.makeDiff(expected, actual1)
			if (similar1) {
				if (diff1.hasDiff()) {
					File diffFile1 = new File(filePath + "/Data Files/Screenshots/${outputName}_rozdiel1.png")
					ImageIO.write(diff1.getMarkedImage(), "PNG", diffFile1)
				}
				KeywordUtil.markPassed("Obrázky sú zhodné alebo s rozdielom do $tolerance % pixelov")
			} else {
				File diffFile1 = new File(filePath + "/Data Files/Screenshots/${outputName}_rozdiel1.png")
				ImageIO.write(diff1.getMarkedImage(), "PNG", diffFile1)
				KeywordUtil.markFailedAndStop("Obrázky sa líšia nad povolenú toleranciu $tolerance % pixelov. Diff: ${diffFile1.path}")
			}
		} else {
			BufferedImage actual2 = ImageIO.read(new File(pathActual2))
			boolean similar1 = areImagesSimilar(expected, actual1, tolerance)
			boolean similar2 = areImagesSimilar(expected, actual2, tolerance)
			ImageDiff diff1 = differ.makeDiff(expected, actual1)
			ImageDiff diff2 = differ.makeDiff(expected, actual2)

			if (similar1) {
				if (diff1.hasDiff()) {
					File diffFile1 = new File(filePath + "/Data Files/Screenshots/${outputName}_rozdiel1.png")
					ImageIO.write(diff1.getMarkedImage(), "PNG", diffFile1)
				}
				KeywordUtil.markPassed("Obrázok #1 je zhodný alebo do tolerancie $tolerance % pixelov")
				return
			} else if (similar2) {
				if (diff2.hasDiff()) {
					File diffFile2 = new File(filePath + "/Data Files/Screenshots/${outputName}_rozdiel2.png")
					ImageIO.write(diff2.getMarkedImage(), "PNG", diffFile2)
				}
				KeywordUtil.markPassed("Obrázok #2 je zhodný alebo do tolerancie $tolerance % pixelov")
				return
			} else {
				File diffFile1 = new File(filePath + "/Data Files/Screenshots/${outputName}_rozdiel1.png")
				File diffFile2 = new File(filePath + "/Data Files/Screenshots/${outputName}_rozdiel2.png")
				ImageIO.write(diff1.getMarkedImage(), "PNG", diffFile1)
				ImageIO.write(diff2.getMarkedImage(), "PNG", diffFile2)
				KeywordUtil.markFailedAndStop("Obrázky sú rozdielne nad povolenú toleranciu $tolerance % pixelov. Diff: ${diffFile1.path}, ${diffFile2.path}")
			}
		}
	}
}


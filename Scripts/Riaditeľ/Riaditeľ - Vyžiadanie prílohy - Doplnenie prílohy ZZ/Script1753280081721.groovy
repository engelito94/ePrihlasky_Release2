import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil
import com.kms.katalon.core.webui.driver.DriverFactory as DriverFactory
import org.openqa.selenium.JavascriptExecutor as JavascriptExecutor
import org.openqa.selenium.WebDriver as WebDriver
import org.openqa.selenium.WebElement as WebElement
import org.openqa.selenium.By as By

Portal portal = new Portal()

portal.prihlasUcet('930571647', 'kBvKxcei4AY8p2seZp2QWw==', GlobalVariable.F2A, true)

/**
 * 
 * TODO vyziadanie priloh
 * 
 */

WebUI.waitForJQueryLoad(30)

WebUI.delay(2)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/PFullText'), 0)

WebUI.setText(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/PFullText'), "Daniela Chudob")

WebUI.click(findTestObject('Object Repository/Zak_test/VydanieRozhodnutiaZS/PButtonSearch'))

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/StavPodana'))

WebUI.click(findTestObject('Object Repository/Zak_test/VyziadaniePrilohy/StavNeuplna'))

WebUI.click(findTestObject('Object Repository/Zak_test/PotvrditNastup/StavPodana'))

WebUI.click(findTestObject('Object Repository/Zak_test/VyziadaniePrilohy/StavNeuplna'))

WebUI.waitForJQueryLoad(30)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Prihlky a rozhodnutia  ePrihlky/button_Zobrazi'),
	0)


WebUI.click(findTestObject('Object Repository/Zak_test/Page_Prihlky a rozhodnutia  ePrihlky/button_Zobrazi'))

WebUI.click(findTestObject('Object Repository/Zak_test/VyziadaniePrilohy/button_VyziadatPrilohu'))

WebUI.waitForJQueryLoad(30)

WebUI.setText(findTestObject('Object Repository/Zak_test/VyziadaniePrilohy/textArea_DovodVyziadania'), "AT test - vyžiadanie prílohy")

WebUI.click(findTestObject('Object Repository/Zak_test/VyziadaniePrilohy/button_OdoslatZiadostPrilohy'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/VyziadaniePrilohy/banner_ZIadostUspesna'),
	'Žiadosť o získanie dodatočnej prílohy ste úspešne odoslali.')

'Odhlásenie'
WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/div_KZ'))

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/a_Odhlsi'))

'Prihlásenie ZZ'
portal.prihlasUcet('930571860', '4Dq6pk30C2ojB+FTFV6r/A==', GlobalVariable.F2A, false)

WebUI.waitForJQueryLoad(30)

WebElement element = null

String elementText = ''

String xpath = ''

boolean najdenaPrihlaska = false

int poradiePrihlasky = 5

WebDriver driver = DriverFactory.getWebDriver()

while (!(najdenaPrihlaska)) {
	xpath = (('//div[' + poradiePrihlasky.toString()) + ']/div/div/div/div[2]/span')

	try {
		elementText = driver.findElement(By.xpath(xpath)).getText()
	}
	catch (NoSuchElementException e) {
		e.message()
	}
	
	if (elementText.equals("Daniela Chudobná")) {
		najdenaPrihlaska = true

		xpath = (('//div[' + poradiePrihlasky.toString()) + ']/div[1]/div[2]/div[2]/a[1]')

	   element = driver.findElement(By.xpath(xpath));
	   ((JavascriptExecutor)driver).executeScript("arguments[0].scrollIntoView(true);", element);
	   
	   //element.click();
		
		//WebUI.executeJavaScript('arguments[0].scrollIntoView(true);', element)
	   
		//klik cez javascript - element mimo viewport
		((JavascriptExecutor)driver).executeScript("arguments[0].click();", element);
	}
	
	poradiePrihlasky++
}

WebUI.click(findTestObject('Object Repository/Zak_test/VyziadaniePrilohy/button_PridatPrilohu'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/VyziadaniePrilohy/select_TypPrilohy'), '1', true)

WebUI.uploadFileWithDragAndDrop(findTestObject('Object Repository/Zak_test/VyziadaniePrilohy/button_NahratiePrilohy'),'C:\\Users\\barcik\\Downloads\\Dokument.pdf')

WebUI.setText(findTestObject('Object Repository/Zak_test/VyziadaniePrilohy/textArea_poznamka'), "AT - priložená príloha")

WebUI.click(findTestObject('Object Repository/Zak_test/VyziadaniePrilohy/button_OdoslatPrilohu'))

WebUI.waitForJQueryLoad(30)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/VyziadaniePrilohy/h1_uspesneNahratie'), 10)

'Odhlásenie'
WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/div_KZ'))

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/a_Odhlsi'))
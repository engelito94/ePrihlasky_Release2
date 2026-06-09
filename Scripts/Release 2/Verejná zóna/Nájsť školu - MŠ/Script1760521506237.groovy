import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.configuration.RunConfiguration as RunConfiguration
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

def filePath = RunConfiguration.getProjectDir()

'vizuálna kontrola MŠ filtrov'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/VerejnaZona/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/a_Domov_govuk-header__link'))

WebUI.waitForJQueryLoad(120)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/VerejnaZona/VymazatFiltreMS'))

WebUI.refresh()

'vizuálna kontrola MŠ filtrov - vlastná adresa'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/VerejnaZona/Page_Njs kolu  ePrihlky/input_Hada poda mojej adresy_hladat-podla-r_99fc84'))

'karta školy - viac info o škole'
WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/VerejnaZona/Page_Njs kolu  ePrihlky/input_Nzov koly alebo jej adresa_nazov-skol_cbc5a9'), 
    'Materská škola pre AT')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/VerejnaZona/Page_Njs kolu  ePrihlky/button_Nzov koly alebo jej adresa_nazov-sko_57678e'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/VerejnaZona/Page_Njs kolu  ePrihlky/span'))

WebUI.waitForJQueryLoad(20)

String img = WebUI.takeFullPageScreenshot(filePath + '/Data Files/Screenshots/img.png')

//String imgOriginal = 'C:\\KatalonProjects\\ePrihlasky\\Data Files\\Screenshots\\MS.png'
String imgOriginal1 = filePath + '/Data Files/Screenshots/MS_1.png'

CustomKeywords.'test.compareImagesWithOptionalSecond'(img, imgOriginal1, null, 'MS')


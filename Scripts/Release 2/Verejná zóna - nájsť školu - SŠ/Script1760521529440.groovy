import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.configuration.RunConfiguration
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

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/VerejnaZona/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/a_Domov_govuk-header__link'))

'vizuálna kontrola SŠ'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/VerejnaZona/Page_Njs kolu  ePrihlky/li_Zkladn koly_nav-item-najst-skolu-SS'))

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Release2/VerejnaZona/NajstSkolu'), 0, FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/VerejnaZona/NajstSkoluNadpis'))

'vizuálna kontrola ŠŠ - rozšírené'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/VerejnaZona/Page_Njs kolu  ePrihlky/span_(nepovinn)_rozsirene-filtre-btn-toggle-text'))

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Release2/VerejnaZona/NajstSkolu'), 0, FailureHandling.STOP_ON_FAILURE)

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/VerejnaZona/SS/Page_Njs kolu  ePrihlky/input_Nzov koly alebo jej adresa_fulltext-input-SS'), 
    'metodova')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/VerejnaZona/SS/Page_Njs kolu  ePrihlky/button_Nzov koly alebo jej adresa_fulltext-_b34249'))

WebUI.waitForJQueryLoad(5)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/VerejnaZona/SS/Page_Njs kolu  ePrihlky/span'))

'kontrola karty školy'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/VerejnaZona/SS/Page_Njs kolu  ePrihlky/span_Odstrni z obbench_material-icons govuk_93fb88'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/VerejnaZona/SS/Page_Njs kolu  ePrihlky/a_Slovensk jazyk a literatra_govuk-link lin_96ee25'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/VerejnaZona/SS/Page_Njs kolu  ePrihlky/span_Menej informci o kole_odbor-header'))

WebUI.waitForJQueryLoad(20)

String img = WebUI.takeFullPageScreenshot(filePath + '/Data Files/Screenshots/img.png')

String imgOriginal = filePath + '/Data Files/Screenshots/SS.png'

CustomKeywords.'test.compareImagesWithOptionalSecond'(img, imgOriginal, null, 'SS')

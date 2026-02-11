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
import com.kms.katalon.core.webui.driver.DriverFactory as DriverFactory
import org.openqa.selenium.JavascriptExecutor as JavascriptExecutor
import org.openqa.selenium.WebDriver as WebDriver

Portal portal = new Portal()

//portal.prihlasUcet2FA(GlobalVariable.login, GlobalVariable.heslo)
portal.prihlasUcet(GlobalVariable.login, GlobalVariable.heslo, GlobalVariable.F2A, false)

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Mj profil  ePrihlky/span_MT'))

WebUI.click(findTestObject('Zak_test/Page_Mj profil  ePrihlky/MojProfil'))

WebUI.delay(1)

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Mj profil  ePrihlky/a_Upravi profil'))

WebUI.delay(1)

WebUI.waitForJQueryLoad(30)

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_et a zabezpeenie  ePrihlky/input_(nepovinn)_input-telefon'), 
    '+421023654789')

if (WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Page_et a zabezpeenie  ePrihlky/input_Severn 874874, 85214, Krpeany, Sloven_e36fcc'), 
    0, FailureHandling.STOP_ON_FAILURE)) {
    WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_et a zabezpeenie  ePrihlky/input_Severn 874874, 85214, Krpeany, Sloven_e36fcc'), 
        0)

    'profil ina adresa'
    WebUI.click(findTestObject('Object Repository/Zak_test/Page_et a zabezpeenie  ePrihlky/input_Severn 874874, 85214, Krpeany, Sloven_e36fcc'))
} else {
    WebUI.click(findTestObject('Object Repository/Zak_test/Page_et a zabezpeenie  ePrihlky/button_Prida adresu'))
}

WebUI.delay(2)

WebUI.waitForJQueryLoad(30)

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_et a zabezpeenie  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input'), 
    'sloven')

WebUI.scrollToElement(findTestObject('Zak_test/Page_et a zabezpeenie  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input'), 
    0)

WebUI.click(findTestObject('Object Repository/Zak_test/Page_et a zabezpeenie  ePrihlky/div_Slovensk republika'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_et a zabezpeenie  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1'), 
    'krp')

WebUI.click(findTestObject('Object Repository/Zak_test/Page_et a zabezpeenie  ePrihlky/div_Krpeany (Martin)'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_et a zabezpeenie  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1_2'), 
    'mas')

WebUI.click(findTestObject('Object Repository/Zak_test/Page_et a zabezpeenie  ePrihlky/div_Krajina        (nepovinn)              _fa333d'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_et a zabezpeenie  ePrihlky/input_(nepovinn)_input-adresaTPSupisneCislo'), 
    '63')

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_et a zabezpeenie  ePrihlky/input_(nepovinn)_input-adresaTPOrientacneCislo'), 
    '874')

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_et a zabezpeenie  ePrihlky/input_(nepovinn)_input-adresaTPPSC'), 
    '85214')

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_et a zabezpeenie  ePrihlky/button_Uloi zmeny'), 0)

WebUI.click(findTestObject('Object Repository/Zak_test/Page_et a zabezpeenie  ePrihlky/button_Uloi zmeny'))

WebUI.waitForJQueryLoad(30)

'Odhlásenie'
WebUI.click(findTestObject('Object Repository/Zak_test/Page_Mj profil  ePrihlky/span_MT'))

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Mj profil  ePrihlky/a_Odhlsi'))


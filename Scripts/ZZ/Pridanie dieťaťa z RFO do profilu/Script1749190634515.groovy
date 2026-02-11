import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import java.lang.System.Logger as Logger
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import groovy.util.logging.Log as Log
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import org.openqa.selenium.JavascriptExecutor as JavascriptExecutor
import com.kms.katalon.core.webui.driver.DriverFactory as DriverFactory
import org.openqa.selenium.WebDriver as WebDriver
import org.openqa.selenium.WebElement as WebElement
import org.openqa.selenium.By as By

Portal portal = new Portal()

portal.prihlasUcet('930570485', 'S0zUR3RetWnmSiXy29cTIg==', GlobalVariable.F2A, false)

WebUI.delay(20)

if (WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/a_Vytvori nov prihlku'), 
    10, FailureHandling.OPTIONAL)) {
    WebUI.delay(1)

    WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/a_Vytvori nov prihlku'))
} else {
    WebUI.delay(1)

    WebUI.click(findTestObject('Object Repository/NovaPrihlaska'))
}

if (WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_add                Prida alie diea'), 
    10, FailureHandling.OPTIONAL)) {
    WebUI.delay(1)

    WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_add                Prida alie diea'))
} else {
    WebUI.waitForJQueryLoad(30)

    WebUI.click(findTestObject('Object Repository/NoveDieta'))
}

WebUI.delay(1)

WebUI.waitForJQueryLoad(30)

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Prida diea  ePrihlky/input_(nepovinn)_input-rodneCislo'), 
    '215313/0155')

WebUI.click(findTestObject('Zak_test/Modal_Button_Dalej'))

WebUI.delay(1)

WebUI.waitForJQueryLoad(30)

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Prida diea  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input'), 
    'slove')

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Prida diea  ePrihlky/div_Krok 22                        ttna prs_8a309c'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Prida diea  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1'), 
    'slove')

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Prida diea  ePrihlky/div_Krok 22                        ttna prs_8a309c_1'))

WebUI.scrollToElement(findTestObject('Zak_test/Modal_Button_PridatDieta'), 0)

WebUI.click(findTestObject('Zak_test/Modal_Button_PridatDieta'))

WebUI.delay(1)

WebUI.waitForJQueryLoad(30)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/Karta_AlzbetaHronova'), 
    0)

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_EH'))

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_Mj profil'))

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Mj profil  ePrihlky/span_Albeta Hronov'), 0)

boolean spravneDieta = WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Mj profil  ePrihlky/span_Albeta Hronov'), 
    'Alžbeta Hronová', FailureHandling.OPTIONAL)

if (spravneDieta) {
    WebDriver driver = DriverFactory.getWebDriver()

    int cislo = 1

    boolean vymazane = false

    while (!(vymazane)) {
        String xpath = ('//tbody/tr[' + cislo.toString()) + ']/td[1]'

        String elementText = driver.findElement(By.xpath(xpath)).getText()

        //println(elementText + " cislo " + cislo)
        if (elementText.equals('Alžbeta Hronová')) {
            xpath = (('//tbody/tr[' + cislo.toString()) + ']/td[2]/button[3]')

            driver.findElement(By.xpath(xpath)).click()

            vymazane = true
        }
        
        cislo++
    }
}

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Mj profil  ePrihlky/span_spene sme vymazali kartu s informciami_572b36'), 
    'Úspešne sme vymazali kartu s informáciami o dieťati Alžbeta Hronová.', FailureHandling.OPTIONAL)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Mj profil  ePrihlky/span_EH'), 0)

'Odhlásenie'
WebUI.click(findTestObject('Object Repository/Zak_test/Page_Mj profil  ePrihlky/span_EH'))

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Mj profil  ePrihlky/a_Odhlsi'))


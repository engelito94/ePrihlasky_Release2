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
import internal.GlobalVariable
import portal.Prihlasovanie

import org.openqa.selenium.Keys as Keys

Prihlasovanie prihlasovanie = new Prihlasovanie()

prihlasovanie.prihlasRiaditela('930570706', 'uEdivOPFtSGvP7ePRyzmOg==', GlobalVariable.F2A, '910020859')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Odbory/2kolo/Page_Prihlky a rozhodnutia  ePrihlky/span'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Odbory/2kolo/Page_Odbory a kritri  ePrihlky/h1_Dokument (1).pdf_title odbory-title'), 
    'Odbory pre 2. kolo')

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/Odbory/2kolo/Page_Odbory a kritri  ePrihlky/button_Odbory pre 2. kolo_btn-otvorit-odbor_330287'), 
    0)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Odbory/2kolo/Page_Odbory a kritri  ePrihlky/button_Odbory pre 2. kolo_btn-otvorit-odbor_330287'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Odbory/2kolo/Page_Odbory a kritri  ePrihlky/div_Uloi_title'), 
    'Otvoriť odbor pre 2. kolo')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Odbory/2kolo/Page_Odbory a kritri  ePrihlky/div_Otvori odbor pre 2. kolo_description-2-kolo'), 
    'Vyberte odbor a zadajte počet voľných miest, ktoré chcete ponúknuť v 2. kole prijímacieho konania pre tento odbor.\n(Počet nesmie prekročiť rozdiel medzi celkovou kapacitou odboru a počtom už prijatých uchádzačov.)')

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/Odbory/2kolo/Page_Odbory a kritri  ePrihlky/select_(nepovinn)_select-modalOtvoritOdborP_c2c744'), 
    '6565e543-a930-4141-ac45-4a96433cd5f4', true)

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Odbory/2kolo/Page_Odbory a kritri  ePrihlky/input_(nepovinn)_input-modalOtvoritOdborPre_84ba16'), 
    '120')

'Pridať odbor'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Odbory/2kolo/Page_Odbory a kritri  ePrihlky/button_Odstrni_btn-otvorit-odbor-pre-2-kolo_dae02c'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Odbory/2kolo/Page_Odbory a kritri  ePrihlky/strong'), 
    'Odbor pre 2. kolo úspešne pridaný.')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Odbory/2kolo/Page_Odbory a kritri  ePrihlky/strong_1'), 
    'Odbory pre 2. kolo nie sú zverejnené')

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/Odbory/2kolo/Page_Odbory a kritri  ePrihlky/button_Otvori odbor pre 2. kolo_btn-zverejn_295ec4'), 
    0)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Odbory/2kolo/Page_Odbory a kritri  ePrihlky/td_Akcia_odbor-poradie'), 
    'O1')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Odbory/2kolo/Page_Odbory a kritri  ePrihlky/td_O1_odbor-kod'), 
    '7902J00')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Odbory/2kolo/Page_Odbory a kritri  ePrihlky/span_O1_odbor-nazov'), 
    'gymnázium - 4 ročné')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Odbory/2kolo/Page_Odbory a kritri  ePrihlky/td_S prijmacou skkou (Slovensk jazyk a lite_7e04b3'), 
    '120')

'Odstránenie odboru'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Odbory/2kolo/Page_Odbory a kritri  ePrihlky/button_Upravi_govuk-button govuk-button--se_559194'))

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/Odbory/2kolo/Page_Odbory a kritri  ePrihlky/button_Odbory pre 2. kolo_btn-otvorit-odbor_330287'), 
    0)

prihlasovanie.odhlasPouzivatela()


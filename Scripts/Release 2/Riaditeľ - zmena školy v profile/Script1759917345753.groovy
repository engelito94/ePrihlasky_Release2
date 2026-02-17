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
import portal.Prihlasovanie as Prihlasovanie
import org.openqa.selenium.Keys as Keys

Prihlasovanie prihlasovanie = new Prihlasovanie()

prihlasovanie.prihlasRiaditela('930570706', 'uEdivOPFtSGvP7ePRyzmOg==', GlobalVariable.F2A, '910019568')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/profilRiaditela/Page_Mj profil  ePrihlky/span_Menu_inicialy-osoby-header'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/profilRiaditela/Page_Mj profil  ePrihlky/a'))

'Zmena školy v profile'
WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/profilRiaditela/Page_Mj profil  ePrihlky/select_(nepovinn)_select-zmenitSkolu'), 
    '910020859', true)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/profilRiaditela/Page_Mj profil  ePrihlky/button_keyboard_arrow_down_zmenit-skolu-btn'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/profilRiaditela/Page_Mj profil  ePrihlky/div_kola_profil-riaditel-skola'), 
    'Gymnázium Metodova', FailureHandling.OPTIONAL)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/profilRiaditela/Page_Mj profil  ePrihlky/span_Gymnzium Metodova_header-eduid'), 
    'EDUID 910020859')

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/profilRiaditela/Page_Mj profil  ePrihlky/select_(nepovinn)_select-zmenitSkolu_1'), 
    '910013679', true)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/profilRiaditela/Page_Mj profil  ePrihlky/button_keyboard_arrow_down_zmenit-skolu-btn'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/profilRiaditela/Page_Mj profil  ePrihlky/div_kola_profil-riaditel-skola_1'), 
    'Súkromná stredná odborná škola')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/profilRiaditela/Page_Mj profil  ePrihlky/span_Skromn stredn odborn kola, Dukelsk 33,_12312b'), 
    'EDUID 910013679')

'Odhlásenie'
prihlasovanie.odhlasPouzivatela()


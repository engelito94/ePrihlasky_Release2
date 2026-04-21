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

prihlasovanie.prihlasRiaditela('930570706', 'uEdivOPFtSGvP7ePRyzmOg==', GlobalVariable.F2A, '910007556')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Prihlky a rozhodnutia  ePrihlky/span'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/span'), 
    'Odbory sú zverejnené')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/a_Odbory_govuk-header__link'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/h1_close_title main-title'), 
    'Odbory a kritériá')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/p_Odbory a kritri_sub-riaditel-kriteria-subtitle'), 
    'Kritériá pre vybraný ročník a odbory.')

'Vybratie odboru pre kritérium'
WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/select_(nepovinn)_select-zvolteOdborSelect'), 
    'd0d9b83e-f2ad-4536-9b82-fc034e95ff0e', true)

'Pridanie kritérií'
WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/input_K1_input-kriterium-nazov-1'), 
    'Kritérium 1')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/input_K1_input-kriterium-vaha-1'), 
    '100')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/div_K1_fake_text_input-1'), 
    '1=1')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/div_K1_slider round'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/span_K1_kriteria-list-btn-text'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/div_K2_slider round'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/input_K2_input-kriterium-nazov-2'), 
    'Kritérium 2')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/input_K2_input-kriterium-vaha-2'), 
    '80')

//WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/span_K2_material-icons'))
//WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/span_Naj()_vypocet_funkcia'))
WebUI.setText(findTestObject('Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/div_K1_fake_text_input-2'), 'Priemer(5+5)')

//WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/span_1'))
//WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/div_K2 Kritrium 2_modal-content'))
//WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/button_Zrui_btn-ulozit-vypocet govuk-button_3cd842'))
WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/div_(nepovinn)_fake_text_input govuk-input _c07cd0'), 
    'K1>K2')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/div_(nepovinn)_fake_text_input govuk-input _c07cd0_1'), 
    'K1,K2')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/button_Zmaza vetko_ulozit-button'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/div_check_circle_govuk-panel__body'), 
    'Systém úspešne uložil zadané kritériá')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/a_Odbory_govuk-header__link'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/select_(nepovinn)_select-zvolteOdborSelect'), 
    'd0d9b83e-f2ad-4536-9b82-fc034e95ff0e', true)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/div_K1_fake_text_input-1'), 
    0)

WebUI.verifyElementPresent(findTestObject('Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/div_K1_fake_text_input-2'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/div_(nepovinn)_fake_text_input govuk-input _c07cd0'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/div_(nepovinn)_fake_text_input govuk-input _c07cd0_1'), 
    0)

prihlasovanie.odhlasPouzivatela()


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

WebUI.openBrowser('')

WebUI.navigateToUrl('https://www.salesforce.com/in/?ir=1')

WebUI.setViewPortSize(2400, 2800)

WebUI.takeFullPageScreenshotAsCheckpoint('VS')

//WebUI.click(findTestObject('Object Repository/salesforce/Page_Salesforce The Customer Company - Sale_f5a4df/pbc-button_Start free trial'))
//
//WebUI.maximizeWindow()
//
//WebUI.switchToWindowTitle('Free CRM Trial: Salesforce 30-Day Trial - Salesforce IN')
//
//WebUI.setText(findTestObject('Object Repository/salesforce/Page_Free CRM Trial Salesforce 30-Day Trial_2b8e59/input_Sign up now to start your free sales _9c5179'), 
//    'abc')
//
//WebUI.setText(findTestObject('Object Repository/salesforce/Page_Free CRM Trial Salesforce 30-Day Trial_2b8e59/input_Enter your first name_UserLastName_1'), 
//    'def')
//
//WebUI.setText(findTestObject('Object Repository/salesforce/Page_Free CRM Trial Salesforce 30-Day Trial_2b8e59/input_Enter your last name_UserEmail_1'), 
//    'def@gmail.com')
//
//WebUI.selectOptionByValue(findTestObject('Object Repository/salesforce/Page_Free CRM Trial Salesforce 30-Day Trial_2b8e59/select_Job Title                           _db0756_1'), 
//    'Sales_Manager_AP', true)
//
//WebUI.setText(findTestObject('Object Repository/salesforce/Page_Free CRM Trial Salesforce 30-Day Trial_2b8e59/input_Select your title_CompanyName_1'), 
//    'def')
//
//WebUI.selectOptionByValue(findTestObject('Object Repository/salesforce/Page_Free CRM Trial Salesforce 30-Day Trial_2b8e59/select_Employees                           _519fae_1'), 
//    '9', true)
//
//WebUI.setText(findTestObject('Object Repository/salesforce/Page_Free CRM Trial Salesforce 30-Day Trial_2b8e59/input_Select employee size_UserPhone_1'), 
//    '4356789023')
//
//WebUI.click(findTestObject('Object Repository/salesforce/Page_Free CRM Trial Salesforce 30-Day Trial_2b8e59/div_Enter your stateprovince_checkbox-ui_1'))
WebUI.closeBrowser()

